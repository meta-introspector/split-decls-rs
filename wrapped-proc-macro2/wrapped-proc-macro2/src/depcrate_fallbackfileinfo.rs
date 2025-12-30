// Generated macro for FileInfo (struct)
macro_rules! Depcrate_fallbackFileInfo {
() => {
// Module: crate::fallback
// Provides: {"FileInfo"}
// Dependencies: {}
# [cfg (all (span_locations , not (fuzzing)))] struct FileInfo { source_text : String , span : Span , lines : Vec < usize > , char_index_to_byte_offset : BTreeMap < usize , usize > , }
};
}
