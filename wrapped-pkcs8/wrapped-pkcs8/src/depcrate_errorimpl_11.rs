// Generated macro for impl_11 (impl)
macro_rules! Depcrate_errorimpl_11 {
() => {
// Module: crate::error
// Provides: {"impl_11"}
// Dependencies: {}
impl From < der :: ErrorKind > for Error { fn from (err : der :: ErrorKind) -> Error { Error :: Asn1 (err . into ()) } }
};
}
