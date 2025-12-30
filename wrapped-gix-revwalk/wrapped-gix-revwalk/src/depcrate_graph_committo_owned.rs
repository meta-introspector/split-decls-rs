// Generated macro for to_owned (module)
macro_rules! Depcrate_graph_committo_owned {
() => {
// Module: crate::graph::commit
// Provides: {"to_owned"}
// Dependencies: {}
# [doc = ""] pub mod to_owned { # [doc = " The error returned by [`to_owned()`][crate::graph::LazyCommit::to_owned()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("A commit could not be decoded during traversal")] Decode (# [from] gix_object :: decode :: Error) , # [error ("Could not find commit position in graph when traversing parents")] CommitGraphParent (# [from] gix_commitgraph :: file :: commit :: Error) , # [error ("Commit-graph time could not be presented as signed integer: {actual}")] CommitGraphTime { actual : u64 } , } }
};
}
