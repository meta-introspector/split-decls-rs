// Generated macro for file_metadata_from_def_id (function)
macro_rules! Depcrate_debuginfo_metadatafile_metadata_from_def_id {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"file_metadata_from_def_id"}
// Dependencies: {}
pub (crate) fn file_metadata_from_def_id < 'll > (cx : & CodegenCx < 'll , '_ > , def_id : Option < DefId > ,) -> DefinitionLocation < 'll > { if let Some (def_id) = def_id && let span = hygiene :: walk_chain_collapsed (cx . tcx . def_span (def_id) , DUMMY_SP) && ! span . is_dummy () { let loc = cx . lookup_debug_loc (span . lo ()) ; (file_metadata (cx , & loc . file) , loc . line) } else { (unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER) } }
};
}
