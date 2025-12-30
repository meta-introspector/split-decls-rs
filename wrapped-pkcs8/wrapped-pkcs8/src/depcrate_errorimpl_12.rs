// Generated macro for impl_12 (impl)
macro_rules! Depcrate_errorimpl_12 {
() => {
// Module: crate::error
// Provides: {"impl_12"}
// Dependencies: {}
# [cfg (feature = "pem")] impl From < pem :: Error > for Error { fn from (err : pem :: Error) -> Error { der :: Error :: from (err) . into () } }
};
}
