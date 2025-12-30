// Generated macro for Index (type)
macro_rules! Depcrate_worktreeIndex {
() => {
// Module: crate::worktree
// Provides: {"Index"}
// Dependencies: {}
# [doc = " A lazily loaded and auto-updated worktree index."] # [cfg (feature = "index")] pub type Index = gix_fs :: SharedFileSnapshot < gix_index :: File > ;
};
}
