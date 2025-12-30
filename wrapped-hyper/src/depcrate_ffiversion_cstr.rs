// Generated macro for VERSION_CSTR (static)
macro_rules! Depcrate_ffiVERSION_CSTR {
() => {
// Module: crate::ffi
// Provides: {"VERSION_CSTR"}
// Dependencies: {}
# [doc = " cbindgen:ignore"] static VERSION_CSTR : & str = concat ! (env ! ("CARGO_PKG_VERSION") , "\0") ;
};
}
