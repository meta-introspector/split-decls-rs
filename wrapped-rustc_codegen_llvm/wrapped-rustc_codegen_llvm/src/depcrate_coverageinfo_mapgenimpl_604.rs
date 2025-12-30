// Generated macro for impl_604 (impl)
macro_rules! Depcrate_coverageinfo_mapgenimpl_604 {
() => {
// Module: crate::coverageinfo::mapgen
// Provides: {"impl_604"}
// Dependencies: {}
impl VirtualFileMapping { fn push_file (& mut self , source_file : & Arc < SourceFile >) -> LocalFileId { self . local_file_table . push (Arc :: clone (source_file)) } # [doc = " Resolves all of the filenames in this local file mapping to a list of"] # [doc = " global file IDs in its CGU, for inclusion in this function's"] # [doc = " `__llvm_covfun` record."] # [doc = ""] # [doc = " The global file IDs are returned as `u32` to make FFI easier."] fn resolve_all (& self , global_file_table : & GlobalFileTable) -> Option < Vec < u32 > > { self . local_file_table . iter () . map (| file | try { let id = global_file_table . get_existing_id (file) ? ; GlobalFileId :: as_u32 (id) }) . collect :: < Option < Vec < _ > > > () } }
};
}
