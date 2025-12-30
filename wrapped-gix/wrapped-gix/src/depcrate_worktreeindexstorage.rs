// Generated macro for IndexStorage (type)
macro_rules! Depcrate_worktreeIndexStorage {
() => {
// Module: crate::worktree
// Provides: {"IndexStorage"}
// Dependencies: {}
# [cfg (feature = "index")] pub (crate) type IndexStorage = gix_features :: threading :: OwnShared < gix_fs :: SharedFileSnapshotMut < gix_index :: File > > ;
};
}
