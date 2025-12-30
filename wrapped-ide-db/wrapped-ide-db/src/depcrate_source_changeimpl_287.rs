// Generated macro for impl_287 (impl)
macro_rules! Depcrate_source_changeimpl_287 {
() => {
// Module: crate::source_change
// Provides: {"impl_287"}
// Dependencies: {}
impl From < IntMap < FileId , TextEdit > > for SourceChange { fn from (source_file_edits : IntMap < FileId , TextEdit >) -> SourceChange { let source_file_edits = source_file_edits . into_iter () . map (| (file_id , edit) | (file_id , (edit , None))) . collect () ; SourceChange { source_file_edits , file_system_edits : Vec :: new () , is_snippet : false , .. SourceChange :: default () } } }
};
}
