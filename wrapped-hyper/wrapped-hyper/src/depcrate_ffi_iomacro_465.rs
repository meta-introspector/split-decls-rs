// Generated macro for macro_465 (macro)
macro_rules! Depcrate_ffi_iomacro_465 {
() => {
// Module: crate::ffi::io
// Provides: {"macro_465"}
// Dependencies: {}
ffi_fn ! { # [doc = " Free an IO handle."] # [doc = ""] # [doc = " This should only be used if the request isn't consumed by"] # [doc = " `hyper_clientconn_handshake`."] fn hyper_io_free (io : * mut hyper_io) { drop (non_null ! (Box :: from_raw (io) ?= ())) ; } }
};
}
