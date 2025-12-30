// Generated macro for t_plus_l (function)
macro_rules! Depcrate_longt_plus_l {
() => {
// Module: crate::long
// Provides: {"t_plus_l"}
// Dependencies: {}
fn t_plus_l (state : [u64 ; COLS]) -> [u64 ; COLS] { let mut state = state ; for nu in 0 .. ROUNDS { add_constant_plus (& mut state , nu as usize) ; apply_s_box (& mut state) ; state = rotate_rows (state) ; mix_columns (& mut state) ; } state }
};
}
