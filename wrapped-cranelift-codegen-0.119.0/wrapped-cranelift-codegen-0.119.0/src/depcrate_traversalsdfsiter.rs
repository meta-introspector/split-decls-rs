// Generated macro for DfsIter (struct)
macro_rules! Depcrate_traversalsDfsIter {
() => {
// Module: crate::traversals
// Provides: {"DfsIter"}
// Dependencies: {}
# [doc = " An iterator that yields pairs of `(Event, ir::Block)` items as it performs a"] # [doc = " depth-first traversal over its associated function."] pub struct DfsIter < 'a > { dfs : & 'a mut Dfs , func : & 'a ir :: Function , }
};
}
