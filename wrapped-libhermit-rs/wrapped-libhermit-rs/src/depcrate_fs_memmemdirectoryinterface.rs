// Generated macro for MemDirectoryInterface (struct)
macro_rules! Depcrate_fs_memMemDirectoryInterface {
() => {
// Module: crate::fs::mem
// Provides: {"MemDirectoryInterface"}
// Dependencies: {}
# [derive (Debug)] pub struct MemDirectoryInterface { # [doc = " Directory entries"] inner : Arc < RwLock < BTreeMap < String , Box < dyn VfsNode + core :: marker :: Send + core :: marker :: Sync > > > > , read_idx : Mutex < usize > , }
};
}
