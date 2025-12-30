// Generated macro for Tx (enum)
macro_rules! Depcrate_ffi_clientTx {
() => {
// Module: crate::ffi::client
// Provides: {"Tx"}
// Dependencies: {}
enum Tx { # [cfg (feature = "http1")] Http1 (conn :: http1 :: SendRequest < crate :: body :: Incoming >) , # [cfg (feature = "http2")] Http2 (conn :: http2 :: SendRequest < crate :: body :: Incoming >) , }
};
}
