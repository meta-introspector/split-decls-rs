// Generated macro for impl_248 (impl)
macro_rules! Depcrate_source_changeimpl_248 {
() => {
// Module: crate::source_change
// Provides: {"impl_248"}
// Dependencies: {}
impl Extend < (FileId , TextEdit) > for SourceChange { fn extend < T : IntoIterator < Item = (FileId , TextEdit) > > (& mut self , iter : T) { self . extend (iter . into_iter () . map (| (file_id , edit) | (file_id , (edit , None)))) } }
};
}
