// Generated macro for Parents (struct)
macro_rules! Depcrate_graph_commitParents {
() => {
// Module: crate::graph::commit
// Provides: {"Parents"}
// Dependencies: {}
# [doc = " An iterator over the parents of a commit."] pub struct Parents < 'graph , 'cache > { backing : Either < gix_object :: CommitRefIter < 'graph > , (& 'cache gix_commitgraph :: Graph , gix_commitgraph :: file :: commit :: Parents < 'cache > ,) , > , }
};
}
