macro_rules! iter_parents {
    () => {
        # [doc = ""] pub mod iter_parents { # [doc = " The error returned by the [`Parents`][super::Parents] iterator."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("An error occurred when parsing commit parents")] DecodeCommit (# [from] gix_object :: decode :: Error) , # [error ("An error occurred when parsing parents from the commit graph")] DecodeCommitGraph (# [from] gix_commitgraph :: file :: commit :: Error) , } }
    };
}

iter_parents!();