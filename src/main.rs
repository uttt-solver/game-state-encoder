use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
enum CellState {
    E, // empty
    O, // O
    X, // X
    D, // dead cell (not affect to other cells)
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CellState::E => " ",
                CellState::O => "O",
                CellState::X => "X",
                CellState::D => "?",
            }
        )?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct GridNotEndState {
    a: CellState,
    b: CellState,
    c: CellState,
    d: CellState,
    e: CellState,
    f: CellState,
    g: CellState,
    h: CellState,
    i: CellState,
}

#[derive(Clone, Copy)]
enum GridState {
    O,
    X,
    E(GridNotEndState),
}

#[derive(Clone, Copy)]
struct BoardNotEndState {
    a: GridState,
    b: GridState,
    c: GridState,
    d: GridState,
    e: GridState,
    f: GridState,
    g: GridState,
    h: GridState,
    i: GridState,
}

#[derive(Clone, Copy)]
enum BoardState {
    O,
    X,
    E(BoardNotEndState),
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! cell {
            ($self:expr, $board:ident, $grid:ident) => {
                match $self {
                    BoardState::O => CellState::O,
                    BoardState::X => CellState::X,
                    BoardState::E(board) => match board.$board {
                        GridState::O => CellState::O,
                        GridState::X => CellState::X,
                        GridState::E(grid) => grid.$grid,
                    },
                }
            };
        }
        macro_rules! grid {
            ($self:expr, $board:ident) => {
                [
                    cell!($self, $board, a),
                    cell!($self, $board, b),
                    cell!($self, $board, c),
                    cell!($self, $board, d),
                    cell!($self, $board, e),
                    cell!($self, $board, f),
                    cell!($self, $board, g),
                    cell!($self, $board, h),
                    cell!($self, $board, i),
                ]
            };
        }
        let board = [
            grid!(self, a),
            grid!(self, b),
            grid!(self, c),
            grid!(self, d),
            grid!(self, e),
            grid!(self, f),
            grid!(self, g),
            grid!(self, h),
            grid!(self, i),
        ];
        writeln!(f, "┏━━━┯━━━┯━━━┓")?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[0][0],
            board[0][1],
            board[0][2],
            board[1][0],
            board[1][1],
            board[1][2],
            board[2][0],
            board[2][1],
            board[2][2]
        )?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[0][3],
            board[0][4],
            board[0][5],
            board[1][3],
            board[1][4],
            board[1][5],
            board[2][3],
            board[2][4],
            board[2][5]
        )?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[0][6],
            board[0][7],
            board[0][8],
            board[1][6],
            board[1][7],
            board[1][8],
            board[2][6],
            board[2][7],
            board[2][8]
        )?;
        writeln!(f, "┠───┼───┼───┨")?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[3][0],
            board[3][1],
            board[3][2],
            board[4][0],
            board[4][1],
            board[4][2],
            board[5][0],
            board[5][1],
            board[5][2]
        )?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[3][3],
            board[3][4],
            board[3][5],
            board[4][3],
            board[4][4],
            board[4][5],
            board[5][3],
            board[5][4],
            board[5][5]
        )?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[3][6],
            board[3][7],
            board[3][8],
            board[4][6],
            board[4][7],
            board[4][8],
            board[5][6],
            board[5][7],
            board[5][8]
        )?;
        writeln!(f, "┠───┼───┼───┨")?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[6][0],
            board[6][1],
            board[6][2],
            board[7][0],
            board[7][1],
            board[7][2],
            board[8][0],
            board[8][1],
            board[8][2]
        )?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[6][3],
            board[6][4],
            board[6][5],
            board[7][3],
            board[7][4],
            board[7][5],
            board[8][3],
            board[8][4],
            board[8][5]
        )?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[6][6],
            board[6][7],
            board[6][8],
            board[7][6],
            board[7][7],
            board[7][8],
            board[8][6],
            board[8][7],
            board[8][8]
        )?;
        writeln!(f, "┗━━━┷━━━┷━━━┛")?;
        Ok(())
    }
}

#[derive(PartialEq)]
enum Player {
    O,
    X,
}

enum GridPosition {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    S,
}

struct GameState {
    board: BoardState,
    turn: Player,
    position: GridPosition,
}

const BASE64_CHARS: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];

fn pack_bools_to_base64(bools: &[bool]) -> String {
    let mut result = String::new();
    let mut current_char = 0u8;
    let mut remaining_bits = 0;
    for &b in bools {
        let bit = if b { 1u8 } else { 0u8 };
        current_char |= bit << (5 - remaining_bits);
        remaining_bits += 1;
        if remaining_bits == 6 {
            result.push(char::from(BASE64_CHARS[current_char as usize]));
            current_char = 0;
            remaining_bits = 0;
        }
    }
    if remaining_bits > 0 {
        result.push(char::from(BASE64_CHARS[current_char as usize]));
    }
    result
}

fn unpack_base64_to_bools(encoded: &str) -> Vec<bool> {
    let mut bools = Vec::new();
    let mut current_char = 0u8;
    let mut remaining_bits = 0;
    for &c in encoded.as_bytes() {
        let index = BASE64_CHARS.iter().position(|&x| x == c as u8).unwrap_or(0);
        current_char = (current_char << 6) | (index as u8);
        remaining_bits += 6;
        while remaining_bits >= 1 {
            let bit = ((current_char >> (remaining_bits - 1)) & 1) == 1;
            bools.push(bit);
            remaining_bits -= 1;
        }
    }
    bools
}

fn pack_grid_position_to_bools(position: &GridPosition) -> [bool; 4] {
    match *position {
        GridPosition::A => [false, false, false, false],
        GridPosition::B => [false, false, false, true],
        GridPosition::C => [false, false, true, false],
        GridPosition::D => [false, false, true, true],
        GridPosition::E => [false, true, false, false],
        GridPosition::F => [false, true, false, true],
        GridPosition::G => [false, true, true, false],
        GridPosition::H => [false, true, true, true],
        GridPosition::I => [true, false, false, false],
        GridPosition::S => [true, false, false, true],
    }
}

fn unpack_grid_position_from_bools(source: &mut Vec<bool>) -> GridPosition {
    match (
        source.pop().unwrap(),
        source.pop().unwrap(),
        source.pop().unwrap(),
        source.pop().unwrap(),
    ) {
        (false, false, false, false) => GridPosition::A,
        (false, false, false, true) => GridPosition::B,
        (false, false, true, false) => GridPosition::C,
        (false, false, true, true) => GridPosition::D,
        (false, true, false, false) => GridPosition::E,
        (false, true, false, true) => GridPosition::F,
        (false, true, true, false) => GridPosition::G,
        (false, true, true, true) => GridPosition::H,
        (true, false, false, false) => GridPosition::I,
        (true, false, false, true) => GridPosition::S,
        _ => panic!("invalid pack"),
    }
}

fn pack_cell_state_to_bools(state: &CellState) -> [bool; 2] {
    match *state {
        CellState::E => [false, false],
        CellState::O => [false, true],
        CellState::X => [true, false],
        CellState::D => [true, true],
    }
}

fn unpack_cell_state_from_bools(source: &mut Vec<bool>) -> CellState {
    match (source.pop().unwrap(), source.pop().unwrap()) {
        (false, false) => CellState::E,
        (false, true) => CellState::O,
        (true, false) => CellState::X,
        (true, true) => CellState::D,
    }
}

fn pack_grid_state_to_bools(state: &GridState) -> Vec<bool> {
    match *state {
        GridState::O => vec![true, false],
        GridState::X => vec![true, true],
        GridState::E(cells) => {
            let mut result = vec![false];
            result.extend(pack_cell_state_to_bools(&cells.a));
            result.extend(pack_cell_state_to_bools(&cells.b));
            result.extend(pack_cell_state_to_bools(&cells.c));
            result.extend(pack_cell_state_to_bools(&cells.d));
            result.extend(pack_cell_state_to_bools(&cells.e));
            result.extend(pack_cell_state_to_bools(&cells.f));
            result.extend(pack_cell_state_to_bools(&cells.g));
            result.extend(pack_cell_state_to_bools(&cells.h));
            result.extend(pack_cell_state_to_bools(&cells.i));
            result
        }
    }
}

fn unpack_grid_state_from_bools(source: &mut Vec<bool>) -> GridState {
    match source.pop().unwrap() {
        true => {
            if source.pop().unwrap() {
                GridState::X
            } else {
                GridState::O
            }
        }
        false => GridState::E(GridNotEndState {
            a: unpack_cell_state_from_bools(source),
            b: unpack_cell_state_from_bools(source),
            c: unpack_cell_state_from_bools(source),
            d: unpack_cell_state_from_bools(source),
            e: unpack_cell_state_from_bools(source),
            f: unpack_cell_state_from_bools(source),
            g: unpack_cell_state_from_bools(source),
            h: unpack_cell_state_from_bools(source),
            i: unpack_cell_state_from_bools(source),
        }),
    }
}

fn pack_game_state_to_bools(game_state: &GameState) -> Vec<bool> {
    let mut result = Vec::new();
    result.push(game_state.turn == Player::X);
    result.extend(pack_grid_position_to_bools(&game_state.position));
    match &game_state.board {
        BoardState::O => result.extend(vec![true, false]),
        BoardState::X => result.extend(vec![true, true]),
        BoardState::E(board_state) => {
            result.push(false);
            result.extend(pack_grid_state_to_bools(&board_state.a));
            result.extend(pack_grid_state_to_bools(&board_state.b));
            result.extend(pack_grid_state_to_bools(&board_state.c));
            result.extend(pack_grid_state_to_bools(&board_state.d));
            result.extend(pack_grid_state_to_bools(&board_state.e));
            result.extend(pack_grid_state_to_bools(&board_state.f));
            result.extend(pack_grid_state_to_bools(&board_state.g));
            result.extend(pack_grid_state_to_bools(&board_state.h));
            result.extend(pack_grid_state_to_bools(&board_state.i));
        }
    }
    result
}

fn unpack_game_state_from_bools(source: &mut Vec<bool>) -> GameState {
    GameState {
        turn: if source.pop().unwrap() {
            Player::X
        } else {
            Player::O
        },
        position: unpack_grid_position_from_bools(source),
        board: match source.pop().unwrap() {
            true => {
                if source.pop().unwrap() {
                    BoardState::X
                } else {
                    BoardState::O
                }
            }
            false => BoardState::E(BoardNotEndState {
                a: unpack_grid_state_from_bools(source),
                b: unpack_grid_state_from_bools(source),
                c: unpack_grid_state_from_bools(source),
                d: unpack_grid_state_from_bools(source),
                e: unpack_grid_state_from_bools(source),
                f: unpack_grid_state_from_bools(source),
                g: unpack_grid_state_from_bools(source),
                h: unpack_grid_state_from_bools(source),
                i: unpack_grid_state_from_bools(source),
            }),
        },
    }
}

fn pack_game_state_to_base64(game_state: &GameState) -> String {
    pack_bools_to_base64(&pack_game_state_to_bools(game_state))
}

fn unpack_game_state_from_base64(encoded: &str) -> GameState {
    let mut bools = unpack_base64_to_bools(encoded);
    bools.reverse();
    unpack_game_state_from_bools(&mut bools)
}

fn main() {
    let empty_grid = GridNotEndState {
        a: CellState::E,
        b: CellState::E,
        c: CellState::E,
        d: CellState::E,
        e: CellState::E,
        f: CellState::E,
        g: CellState::E,
        h: CellState::E,
        i: CellState::E,
    };
    let empty_board = BoardNotEndState {
        a: GridState::E(empty_grid),
        b: GridState::E(empty_grid),
        c: GridState::E(empty_grid),
        d: GridState::E(empty_grid),
        e: GridState::E(empty_grid),
        f: GridState::E(empty_grid),
        g: GridState::E(empty_grid),
        h: GridState::E(empty_grid),
        i: GridState::E(empty_grid),
    };
    println!(
        "{}",
        pack_game_state_to_base64(&unpack_game_state_from_base64(&pack_game_state_to_base64(
            &GameState {
                turn: Player::O,
                position: GridPosition::S,
                board: BoardState::E(empty_board),
            }
        )))
    );
    macro_rules! print_initial_game_state {
        ($position:ident, $board:ident, $grid:ident) => {
            let grid = GridState::E(GridNotEndState {
                $grid: CellState::O,
                ..empty_grid
            });
            let board = BoardState::E(BoardNotEndState {
                $board: grid,
                ..empty_board
            });
            println!(
                "{}",
                pack_game_state_to_base64(&GameState {
                    turn: Player::X,
                    position: GridPosition::$position,
                    board,
                })
            );
        };
    }
    macro_rules! print_initial_game_states {
        ($position:ident, $board:ident) => {
            print_initial_game_state!($position, $board, a);
            print_initial_game_state!($position, $board, b);
            print_initial_game_state!($position, $board, c);
            print_initial_game_state!($position, $board, d);
            print_initial_game_state!($position, $board, e);
            print_initial_game_state!($position, $board, f);
            print_initial_game_state!($position, $board, g);
            print_initial_game_state!($position, $board, h);
            print_initial_game_state!($position, $board, i);
        };
    }
    print_initial_game_states!(A, a);
    print_initial_game_states!(B, b);
    print_initial_game_states!(C, c);
    print_initial_game_states!(D, d);
    print_initial_game_states!(E, e);
    print_initial_game_states!(F, f);
    print_initial_game_states!(G, g);
    print_initial_game_states!(H, h);
    print_initial_game_states!(I, i);
}
