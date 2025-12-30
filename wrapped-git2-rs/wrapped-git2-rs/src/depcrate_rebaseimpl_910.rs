// Generated macro for impl_910 (impl)
macro_rules! Depcrate_rebaseimpl_910 {
() => {
// Module: crate::rebase
// Provides: {"impl_910"}
// Dependencies: {}
impl RebaseOperationType { # [doc = " Convert from the int into an enum. Returns None if invalid."] pub fn from_raw (raw : raw :: git_rebase_operation_t) -> Option < RebaseOperationType > { match raw { raw :: GIT_REBASE_OPERATION_PICK => Some (RebaseOperationType :: Pick) , raw :: GIT_REBASE_OPERATION_REWORD => Some (RebaseOperationType :: Reword) , raw :: GIT_REBASE_OPERATION_EDIT => Some (RebaseOperationType :: Edit) , raw :: GIT_REBASE_OPERATION_SQUASH => Some (RebaseOperationType :: Squash) , raw :: GIT_REBASE_OPERATION_FIXUP => Some (RebaseOperationType :: Fixup) , raw :: GIT_REBASE_OPERATION_EXEC => Some (RebaseOperationType :: Exec) , _ => None , } } }
};
}
