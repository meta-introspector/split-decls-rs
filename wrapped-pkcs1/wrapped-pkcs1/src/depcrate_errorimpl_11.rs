// Generated macro for impl_11 (impl)
macro_rules! Depcrate_errorimpl_11 {
() => {
// Module: crate::error
// Provides: {"impl_11"}
// Dependencies: {}
# [cfg (feature = "pem")] impl From < pem :: Error > for Error { fn from (err : pem :: Error) -> Error { der :: Error :: from (err) . into () } }
};
}
