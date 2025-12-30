// Generated macro for LazyCommit (struct)
macro_rules! Depcrate_graphLazyCommit {
() => {
// Module: crate::graph
// Provides: {"LazyCommit"}
// Dependencies: {}
# [doc = " A commit that provides access to graph-related information, on demand."] # [doc = ""] # [doc = " The owned version of this type is called [`Commit`] and can be obtained by calling [`LazyCommit::to_owned()`]."] pub struct LazyCommit < 'graph , 'cache > { backing : Either < & 'graph [u8] , (& 'cache gix_commitgraph :: Graph , gix_commitgraph :: Position) > , }
};
}
