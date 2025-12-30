// Generated macro for compress (function)
macro_rules! Depcrate_longcompress {
() => {
// Module: crate::long
// Provides: {"compress"}
// Dependencies: {}
pub (crate) fn compress (prev_vector : & mut [u64 ; COLS] , message_block : & [u8 ; 128]) { let message_u64 = read_u64s_be :: < 128 , COLS > (message_block) ; let m_xor_p = xor (* prev_vector , message_u64) ; let t_xor_mp = t_xor_l (m_xor_p) ; let t_plus_m = t_plus_l (message_u64) ; * prev_vector = xor (xor (t_xor_mp , t_plus_m) , * prev_vector) ; }
};
}
