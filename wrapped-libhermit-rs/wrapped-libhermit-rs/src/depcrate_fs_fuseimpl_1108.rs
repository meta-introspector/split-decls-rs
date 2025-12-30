// Generated macro for impl_1108 (impl)
macro_rules! Depcrate_fs_fuseimpl_1108 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1108"}
// Dependencies: {}
impl FuseDirectoryHandle { pub fn new (name : Option < String >) -> Self { Self { name , read_position : Mutex :: new (0) , } } }
};
}
