// Generated macro for impl_1704 (impl)
macro_rules! Depcrate_shims_io_errorimpl_1704 {
() => {
// Module: crate::shims::io_error
// Provides: {"impl_1704"}
// Dependencies: {}
impl From < io :: Error > for IoError { fn from (value : io :: Error) -> Self { IoError :: HostError (value) } }
};
}
