// Generated macro for get_source_range (function)
macro_rules! Depcrate_sourceget_source_range {
() => {
// Module: crate::source
// Provides: {"get_source_range"}
// Dependencies: {}
fn get_source_range (sm : & SourceMap , sp : Range < BytePos >) -> Option < SourceFileRange > { let start = sm . lookup_byte_offset (sp . start) ; let end = sm . lookup_byte_offset (sp . end) ; if ! Arc :: ptr_eq (& start . sf , & end . sf) || start . pos > end . pos { return None ; } sm . ensure_source_file_source_present (& start . sf) ; let range = start . pos . to_usize () .. end . pos . to_usize () ; Some (SourceFileRange { sf : start . sf , range }) }
};
}
