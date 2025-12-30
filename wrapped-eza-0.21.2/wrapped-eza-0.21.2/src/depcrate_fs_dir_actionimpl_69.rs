// Generated macro for impl_69 (impl)
macro_rules! Depcrate_fs_dir_actionimpl_69 {
() => {
// Module: crate::fs::dir_action
// Provides: {"impl_69"}
// Dependencies: {}
impl DirAction { # [doc = " Gets the recurse options, if this dir action has any."] pub fn recurse_options (self) -> Option < RecurseOptions > { match self { Self :: Recurse (o) => Some (o) , _ => None , } } # [doc = " Whether to treat directories as regular files or not."] pub fn treat_dirs_as_files (self) -> bool { match self { Self :: AsFile => true , Self :: Recurse (o) => o . tree , Self :: List => false , } } }
};
}
