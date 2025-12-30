// Generated macro for quiche_version (function)
macro_rules! Depcrate_ffiquiche_version {
() => {
// Module: crate::ffi
// Provides: {"quiche_version"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_version () -> * const u8 { static VERSION : & str = concat ! (env ! ("CARGO_PKG_VERSION") , "\0") ; VERSION . as_ptr () }
};
}
