// Generated macro for impl_261 (impl)
macro_rules! Depcrate_source_changeimpl_261 {
() => {
// Module: crate::source_change
// Provides: {"impl_261"}
// Dependencies: {}
impl From < FileSystemEdit > for SourceChange { fn from (edit : FileSystemEdit) -> SourceChange { SourceChange { source_file_edits : Default :: default () , file_system_edits : vec ! [edit] , is_snippet : false , .. SourceChange :: default () } } }
};
}
