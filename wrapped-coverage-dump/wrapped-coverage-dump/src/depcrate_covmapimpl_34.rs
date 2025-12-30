// Generated macro for impl_34 (impl)
macro_rules! Depcrate_covmapimpl_34 {
() => {
// Module: crate::covmap
// Provides: {"impl_34"}
// Dependencies: {}
impl FilenameTables { pub (crate) fn lookup (& self , filenames_hash : u64 , global_file_id : usize) -> Option < & str > { let table = self . map . get (& filenames_hash) ? ; let filename = table . get (global_file_id) ? ; Some (filename) } }
};
}
