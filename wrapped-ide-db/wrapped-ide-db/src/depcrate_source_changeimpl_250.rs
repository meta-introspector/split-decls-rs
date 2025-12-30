// Generated macro for impl_250 (impl)
macro_rules! Depcrate_source_changeimpl_250 {
() => {
// Module: crate::source_change
// Provides: {"impl_250"}
// Dependencies: {}
impl Extend < FileSystemEdit > for SourceChange { fn extend < T : IntoIterator < Item = FileSystemEdit > > (& mut self , iter : T) { iter . into_iter () . for_each (| edit | self . push_file_system_edit (edit)) ; } }
};
}
