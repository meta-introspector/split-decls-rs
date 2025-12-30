// Generated macro for align_buf_mut (function)
macro_rules! Depcrate_typesalign_buf_mut {
() => {
// Module: crate::types
// Provides: {"align_buf_mut"}
// Dependencies: {}
pub fn align_buf_mut < M : Marshal > (a : & mut [u8]) -> & mut [u8] { let p = a . as_ptr () as usize ; let n = align_up (p , M :: ALIGN) ; & mut a [(n - p) ..] }
};
}
