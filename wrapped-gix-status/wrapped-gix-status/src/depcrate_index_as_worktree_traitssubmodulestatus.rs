// Generated macro for SubmoduleStatus (trait)
macro_rules! Depcrate_index_as_worktree_traitsSubmoduleStatus {
() => {
// Module: crate::index_as_worktree::traits
// Provides: {"SubmoduleStatus"}
// Dependencies: {}
# [doc = " Determine the status of a submodule, which always indicates that it changed if present."] pub trait SubmoduleStatus { # [doc = " The status result, describing in which way the submodule changed."] type Output ; # [doc = " A custom error that may occur while computing the submodule status."] type Error : std :: error :: Error + Send + Sync + 'static ; # [doc = " Compute the status of the submodule at `entry` and `rela_path`, or return `None` if no change was detected."] fn status (& mut self , entry : & gix_index :: Entry , rela_path : & BStr) -> Result < Option < Self :: Output > , Self :: Error > ; }
};
}
