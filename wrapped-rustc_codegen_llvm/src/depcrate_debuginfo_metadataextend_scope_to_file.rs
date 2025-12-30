// Generated macro for extend_scope_to_file (function)
macro_rules! Depcrate_debuginfo_metadataextend_scope_to_file {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"extend_scope_to_file"}
// Dependencies: {}
# [doc = " Creates an \"extension\" of an existing `DIScope` into another file."] pub (crate) fn extend_scope_to_file < 'll > (cx : & CodegenCx < 'll , '_ > , scope_metadata : & 'll DIScope , file : & SourceFile ,) -> & 'll DILexicalBlock { let file_metadata = file_metadata (cx , file) ; unsafe { llvm :: LLVMDIBuilderCreateLexicalBlockFile (DIB (cx) , scope_metadata , file_metadata , 0u32 ,) } }
};
}
