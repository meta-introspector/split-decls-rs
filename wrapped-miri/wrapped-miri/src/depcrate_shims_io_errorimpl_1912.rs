// Generated macro for impl_1912 (impl)
macro_rules! Depcrate_shims_io_errorimpl_1912 {
() => {
// Module: crate::shims::io_error
// Provides: {"impl_1912"}
// Dependencies: {}
impl From < io :: ErrorKind > for IoError { fn from (value : io :: ErrorKind) -> Self { IoError :: HostError (value . into ()) } }
};
}
