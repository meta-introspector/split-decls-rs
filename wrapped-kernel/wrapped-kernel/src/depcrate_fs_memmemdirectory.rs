// Generated macro for MemDirectory (struct)
macro_rules! Depcrate_fs_memMemDirectory {
() => {
// Module: crate::fs::mem
// Provides: {"MemDirectory"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct MemDirectory { inner : Arc < RwLock < BTreeMap < String , Box < dyn VfsNode + core :: marker :: Send + core :: marker :: Sync > > > > , attr : FileAttr , }
};
}
