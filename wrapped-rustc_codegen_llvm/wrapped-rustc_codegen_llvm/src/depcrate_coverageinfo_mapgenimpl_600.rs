// Generated macro for impl_600 (impl)
macro_rules! Depcrate_coverageinfo_mapgenimpl_600 {
() => {
// Module: crate::coverageinfo::mapgen
// Provides: {"impl_600"}
// Dependencies: {}
impl GlobalFileTable { # [doc = " Builds a \"global file table\" for this CGU, mapping numeric IDs to"] # [doc = " path strings."] fn build < 'a > (tcx : TyCtxt < '_ > , all_files : impl Iterator < Item = & 'a SourceFile >) -> Self { let mut raw_file_table = FxIndexMap :: default () ; for file in all_files { raw_file_table . entry (file . stable_id) . or_insert_with (| | { file . name . for_scope (tcx . sess , RemapPathScopeComponents :: MACRO) . to_string_lossy () . into_owned () }) ; } let mut table = Vec :: with_capacity (raw_file_table . len () + 1) ; let base_dir = tcx . sess . opts . working_dir . for_scope (tcx . sess , RemapPathScopeComponents :: MACRO) . to_string_lossy () ; table . push (base_dir . as_ref ()) ; table . extend (raw_file_table . values () . map (| name | name . as_str ())) ; let filenames_buffer = llvm_cov :: write_filenames_to_buffer (& table) ; let filenames_hash = llvm_cov :: hash_bytes (& filenames_buffer) ; Self { raw_file_table , filenames_buffer , filenames_hash } } fn get_existing_id (& self , file : & SourceFile) -> Option < GlobalFileId > { let raw_id = self . raw_file_table . get_index_of (& file . stable_id) ? ; Some (GlobalFileId :: from_usize (raw_id + 1)) } }
};
}
