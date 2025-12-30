// Generated macro for unknown_file_metadata (function)
macro_rules! Depcrate_debuginfo_metadataunknown_file_metadata {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"unknown_file_metadata"}
// Dependencies: {}
fn unknown_file_metadata < 'll > (cx : & CodegenCx < 'll , '_ >) -> & 'll DIFile { debug_context (cx) . created_files . borrow_mut () . entry (None) . or_insert_with (| | { create_file (DIB (cx) , "<unknown>" , "" , "" , llvm :: ChecksumKind :: None , None) }) }
};
}
