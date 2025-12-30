// Generated macro for impl_249 (impl)
macro_rules! Depcrate_source_changeimpl_249 {
() => {
// Module: crate::source_change
// Provides: {"impl_249"}
// Dependencies: {}
impl Extend < (FileId , (TextEdit , Option < SnippetEdit >)) > for SourceChange { fn extend < T : IntoIterator < Item = (FileId , (TextEdit , Option < SnippetEdit >)) > > (& mut self , iter : T ,) { iter . into_iter () . for_each (| (file_id , (edit , snippet_edit)) | { self . insert_source_and_snippet_edit (file_id , edit , snippet_edit) }) ; } }
};
}
