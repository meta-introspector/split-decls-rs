// Generated macro for impl_297 (impl)
macro_rules! Depcrate_source_changeimpl_297 {
() => {
// Module: crate::source_change
// Provides: {"impl_297"}
// Dependencies: {}
impl From < FileSystemEdit > for SourceChange { fn from (edit : FileSystemEdit) -> SourceChange { SourceChange { source_file_edits : Default :: default () , file_system_edits : vec ! [edit] , is_snippet : false , .. SourceChange :: default () } } }
};
}
