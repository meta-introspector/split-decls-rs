// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl AutoRemove { fn execute_best_effort (self , directory_to_potentially_delete : & Path) -> Option < PathBuf > { match self { AutoRemove :: Tempfile => None , AutoRemove :: TempfileAndEmptyParentDirectoriesUntil { boundary_directory } => { remove_dir :: empty_upward_until_boundary (directory_to_potentially_delete , & boundary_directory) . ok () ; Some (boundary_directory) } } } }
};
}
