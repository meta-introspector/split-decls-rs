// Generated macro for impl_1104 (impl)
macro_rules! Depcrate_fs_fuseimpl_1104 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1104"}
// Dependencies: {}
impl FuseFileHandle { pub fn new () -> Self { Self (Arc :: new (Mutex :: new (FuseFileHandleInner :: new ()))) } }
};
}
