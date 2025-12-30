// Generated macro for rotate_rows (function)
macro_rules! Depcrate_longrotate_rows {
() => {
// Module: crate::long
// Provides: {"rotate_rows"}
// Dependencies: {}
fn rotate_rows (state : [u64 ; COLS]) -> [u64 ; COLS] { const SHIFTS : [usize ; 8] = [0 , 1 , 2 , 3 , 4 , 5 , 6 , 11] ; array :: from_fn (| col | { let rotated_bytes = array :: from_fn (| row | { let shift = SHIFTS [row] ; let src_col = (col + COLS - shift) % COLS ; let src_bytes = state [src_col] . to_be_bytes () ; src_bytes [row] }) ; u64 :: from_be_bytes (rotated_bytes) }) }
};
}
