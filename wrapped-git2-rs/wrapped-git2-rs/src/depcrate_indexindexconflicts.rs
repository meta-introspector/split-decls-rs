// Generated macro for IndexConflicts (struct)
macro_rules! Depcrate_indexIndexConflicts {
() => {
// Module: crate::index
// Provides: {"IndexConflicts"}
// Dependencies: {}
# [doc = " An iterator over the conflicting entries in an index"] pub struct IndexConflicts < 'index > { conflict_iter : * mut raw :: git_index_conflict_iterator , _marker : marker :: PhantomData < & 'index Index > , }
};
}
