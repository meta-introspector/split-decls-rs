// Generated macro for impl_32 (impl)
macro_rules! Depcrate_testingimpl_32 {
() => {
// Module: crate::testing
// Provides: {"impl_32"}
// Dependencies: {}
impl FileIdCache for TestCache { fn cached_file_id (& self , path : & Path) -> Option < impl AsRef < FileId > > { self . paths . get (path) } fn add_path (& mut self , path : & Path , recursive_mode : RecursiveMode) { for (file_path , file_id) in & self . file_system { if file_path == path || (file_path . starts_with (path) && recursive_mode == RecursiveMode :: Recursive) { self . paths . insert (file_path . clone () , * file_id) ; } } } fn remove_path (& mut self , path : & Path) { self . paths . remove (path) ; } }
};
}
