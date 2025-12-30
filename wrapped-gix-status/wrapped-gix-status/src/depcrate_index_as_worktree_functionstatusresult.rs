// Generated macro for StatusResult (type)
macro_rules! Depcrate_index_as_worktree_functionStatusResult {
() => {
// Module: crate::index_as_worktree::function
// Provides: {"StatusResult"}
// Dependencies: {}
type StatusResult < 'index , T , U > = Result < (& 'index gix_index :: Entry , usize , & 'index BStr , EntryStatus < T , U >) , Error > ;
};
}
