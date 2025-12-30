// Generated macro for macro_373 (macro)
macro_rules! Depcrate_ffi_bodymacro_373 {
() => {
// Module: crate::ffi::body
// Provides: {"macro_373"}
// Dependencies: {}
ffi_fn ! { # [doc = " Free this buffer."] # [doc = ""] # [doc = " This should be used for any buffer once it is no longer needed."] fn hyper_buf_free (buf : * mut hyper_buf) { drop (unsafe { Box :: from_raw (buf) }) ; } }
};
}
