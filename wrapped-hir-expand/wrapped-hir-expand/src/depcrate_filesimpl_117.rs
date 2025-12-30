// Generated macro for impl_117 (impl)
macro_rules! Depcrate_filesimpl_117 {
() => {
// Module: crate::files
// Provides: {"impl_117"}
// Dependencies: {}
impl From < FilePosition > for HirFilePosition { fn from (value : FilePosition) -> Self { HirFilePosition { file_id : value . file_id . into () , offset : value . offset } } }
};
}
