// Generated macro for macro_372 (macro)
macro_rules! Depcrate_ffi_bodymacro_372 {
() => {
// Module: crate::ffi::body
// Provides: {"macro_372"}
// Dependencies: {}
ffi_fn ! { # [doc = " Get the length of the bytes this buffer contains."] fn hyper_buf_len (buf : * const hyper_buf) -> size_t { unsafe { (* buf) . 0 . len () } } }
};
}
