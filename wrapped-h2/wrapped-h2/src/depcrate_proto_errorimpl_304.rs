// Generated macro for impl_304 (impl)
macro_rules! Depcrate_proto_errorimpl_304 {
() => {
// Module: crate::proto::error
// Provides: {"impl_304"}
// Dependencies: {}
impl From < io :: ErrorKind > for Error { fn from (src : io :: ErrorKind) -> Self { Error :: Io (src , None) } }
};
}
