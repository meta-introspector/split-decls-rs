// Generated macro for impl_1146 (impl)
macro_rules! Depcrate_fs_memimpl_1146 {
() => {
// Module: crate::fs::mem
// Provides: {"impl_1146"}
// Dependencies: {}
impl VfsNode for RamFile { fn get_kind (& self) -> NodeKind { NodeKind :: File } fn get_object (& self) -> io :: Result < Arc < async_lock :: RwLock < dyn ObjectInterface > > > { Ok (Arc :: new (async_lock :: RwLock :: new (RamFileInterface :: new (self . data . clone () ,)))) } fn get_file_attributes (& self) -> io :: Result < FileAttr > { block_on (async { Ok (self . data . read () . await . attr) } , None) } fn traverse_lstat (& self , components : & mut Vec < & str >) -> io :: Result < FileAttr > { if components . is_empty () { self . get_file_attributes () } else { Err (Errno :: Badf) } } fn traverse_stat (& self , components : & mut Vec < & str >) -> io :: Result < FileAttr > { if components . is_empty () { self . get_file_attributes () } else { Err (Errno :: Badf) } } }
};
}
