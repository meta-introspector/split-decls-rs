// Generated macro for impl_156 (impl)
macro_rules! Depcrate_errorimpl_156 {
() => {
// Module: crate::error
// Provides: {"impl_156"}
// Dependencies: {}
# [cfg (feature = "oid")] impl From < const_oid :: Error > for Error { fn from (_ : const_oid :: Error) -> Error { ErrorKind :: OidMalformed . into () } }
};
}
