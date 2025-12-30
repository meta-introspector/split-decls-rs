// Generated macro for SourceChange (struct)
macro_rules! Depcrate_source_changeSourceChange {
() => {
// Module: crate::source_change
// Provides: {"SourceChange"}
// Dependencies: {}
# [derive (Default , Debug , Clone)] pub struct SourceChange { pub source_file_edits : IntMap < FileId , (TextEdit , Option < SnippetEdit >) > , pub file_system_edits : Vec < FileSystemEdit > , pub is_snippet : bool , pub annotations : FxHashMap < ChangeAnnotationId , ChangeAnnotation > , next_annotation_id : u32 , }
};
}
