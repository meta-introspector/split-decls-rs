// Generated macro for impl_1149 (impl)
macro_rules! Depcrate_fs_memimpl_1149 {
() => {
// Module: crate::fs::mem
// Provides: {"impl_1149"}
// Dependencies: {}
impl MemDirectoryInterface { pub fn new (inner : Arc < RwLock < BTreeMap < String , Box < dyn VfsNode + core :: marker :: Send + core :: marker :: Sync > > > , > ,) -> Self { Self { inner , read_idx : Mutex :: new (0) , } } }
};
}
