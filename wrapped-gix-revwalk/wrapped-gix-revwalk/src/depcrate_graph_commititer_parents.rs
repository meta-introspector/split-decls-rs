// Generated macro for iter_parents (module)
macro_rules! Depcrate_graph_commititer_parents {
() => {
// Module: crate::graph::commit
// Provides: {"iter_parents"}
// Dependencies: {}
# [doc = ""] pub mod iter_parents { # [doc = " The error returned by the [`Parents`][super::Parents] iterator."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("An error occurred when parsing commit parents")] DecodeCommit (# [from] gix_object :: decode :: Error) , # [error ("An error occurred when parsing parents from the commit graph")] DecodeCommitGraph (# [from] gix_commitgraph :: file :: commit :: Error) , } }
};
}
