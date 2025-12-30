// Generated macro for impl_157 (impl)
macro_rules! Depcrate_errorimpl_157 {
() => {
// Module: crate::error
// Provides: {"impl_157"}
// Dependencies: {}
# [cfg (feature = "pem")] impl From < pem :: Error > for Error { fn from (err : pem :: Error) -> Error { ErrorKind :: Pem (err) . into () } }
};
}
