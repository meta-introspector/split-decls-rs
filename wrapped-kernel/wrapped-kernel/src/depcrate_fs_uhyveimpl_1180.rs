// Generated macro for impl_1180 (impl)
macro_rules! Depcrate_fs_uhyveimpl_1180 {
() => {
// Module: crate::fs::uhyve
// Provides: {"impl_1180"}
// Dependencies: {}
impl UhyveFileHandle { pub fn new (fd : i32) -> Self { Self (Arc :: new (Mutex :: new (UhyveFileHandleInner :: new (fd)))) } }
};
}
