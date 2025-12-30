// Generated macro for impl_411 (impl)
macro_rules! Depcrate_index_writeimpl_411 {
() => {
// Module: crate::index::write
// Provides: {"impl_411"}
// Dependencies: {}
impl From < ProgressId > for gix_features :: progress :: Id { fn from (v : ProgressId) -> Self { match v { ProgressId :: IndexObjects => * b"IWIO" , ProgressId :: DecompressedBytes => * b"IWDB" , ProgressId :: ResolveObjects => * b"IWRO" , ProgressId :: DecodedBytes => * b"IWDB" , ProgressId :: IndexBytesWritten => * b"IWBW" , } } }
};
}
