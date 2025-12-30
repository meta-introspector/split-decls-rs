// Generated macro for impl_1094 (impl)
macro_rules! Depcrate_fs_fuseimpl_1094 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1094"}
// Dependencies: {}
impl From < FuseError > for Errno { fn from (value : FuseError) -> Self { match value { FuseError :: VirtqError (virtq_error) => virtq_error . into () , FuseError :: IOError (io_error) => io_error , } } }
};
}
