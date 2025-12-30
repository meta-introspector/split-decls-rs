// Generated macro for impl_71 (impl)
macro_rules! Depcrate_fs_dir_actionimpl_71 {
() => {
// Module: crate::fs::dir_action
// Provides: {"impl_71"}
// Dependencies: {}
impl RecurseOptions { # [doc = " Returns whether a directory of the given depth would be too deep."] pub fn is_too_deep (self , depth : usize) -> bool { match self . max_depth { None => false , Some (d) => d <= depth , } } }
};
}
