// Generated macro for impl_69 (impl)
macro_rules! Depcrate_blob_pipelineimpl_69 {
() => {
// Module: crate::blob::pipeline
// Provides: {"impl_69"}
// Dependencies: {}
impl WorktreeRoots { # [doc = " Return the root path for the given `kind`"] pub fn by_kind (& self , kind : ResourceKind) -> Option < & Path > { match kind { ResourceKind :: CurrentOrOurs => self . current_root . as_deref () , ResourceKind :: CommonAncestorOrBase => self . common_ancestor_root . as_deref () , ResourceKind :: OtherOrTheirs => self . other_root . as_deref () , } } # [doc = " Return `true` if all worktree roots are unset."] pub fn is_unset (& self) -> bool { self . current_root . is_none () && self . other_root . is_none () && self . common_ancestor_root . is_none () } }
};
}
