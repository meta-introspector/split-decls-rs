// Generated macro for with_output_vec (function)
macro_rules! Depcratewith_output_vec {
() => {
// Module: crate
// Provides: {"with_output_vec"}
// Dependencies: {}
# [doc = " Wrap a closure that writes at most `max_output` bytes to fill a vector."] # [doc = " It must return the number of bytes written."] # [doc = ""] # [doc = " Safety: `F` must not write more than `max_output` bytes and must return"] # [doc = " the number of bytes written."] # [allow (clippy :: unwrap_used)] unsafe fn with_output_vec < F > (max_output : usize , func : F) -> Vec < u8 > where F : FnOnce (* mut u8) -> usize , { unsafe { with_output_vec_fallible (max_output , | out_buf | Some (func (out_buf))) . unwrap () } }
};
}
