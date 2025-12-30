// Generated macro for macro_390 (macro)
macro_rules! Depcrate_ffi_clientmacro_390 {
() => {
// Module: crate::ffi::client
// Provides: {"macro_390"}
// Dependencies: {}
ffi_fn ! { # [doc = " Free a `hyper_clientconn *`."] # [doc = ""] # [doc = " This should be used for any connection once it is no longer needed."] fn hyper_clientconn_free (conn : * mut hyper_clientconn) { drop (non_null ! { Box :: from_raw (conn) ?= () }) ; } }
};
}
