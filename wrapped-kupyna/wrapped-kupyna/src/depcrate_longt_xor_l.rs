// Generated macro for t_xor_l (function)
macro_rules! Depcrate_longt_xor_l {
() => {
// Module: crate::long
// Provides: {"t_xor_l"}
// Dependencies: {}
pub (crate) fn t_xor_l (state : [u64 ; COLS]) -> [u64 ; COLS] { let mut state = state ; for nu in 0 .. ROUNDS { add_constant_xor (& mut state , nu as usize) ; apply_s_box (& mut state) ; state = rotate_rows (state) ; mix_columns (& mut state) ; } state }
};
}
