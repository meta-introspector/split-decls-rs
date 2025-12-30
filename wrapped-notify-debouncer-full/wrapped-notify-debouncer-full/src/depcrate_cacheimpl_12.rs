// Generated macro for impl_12 (impl)
macro_rules! Depcrate_cacheimpl_12 {
() => {
// Module: crate::cache
// Provides: {"impl_12"}
// Dependencies: {}
impl FileIdCache for NoCache { fn cached_file_id (& self , _path : & Path) -> Option < impl AsRef < FileId > > { Option :: < & FileId > :: None } fn add_path (& mut self , _path : & Path , _recursive_mode : RecursiveMode) { } fn remove_path (& mut self , _path : & Path) { } }
};
}
