// Generated macro for errors (module)
macro_rules! Depcrate_grapherrors {
() => {
// Module: crate::graph
// Provides: {"errors"}
// Dependencies: {}
mod errors { # [doc = ""] pub mod insert_parents { use crate :: graph :: commit :: iter_parents ; # [doc = " The error returned by [`insert_parents()`](crate::Graph::insert_parents())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Lookup (# [from] gix_object :: find :: existing_iter :: Error) , # [error ("A commit could not be decoded during traversal")] Decode (# [from] gix_object :: decode :: Error) , # [error (transparent)] Parent (# [from] iter_parents :: Error) , } } # [doc = ""] pub mod get_or_insert_default { use crate :: graph :: commit :: to_owned ; # [doc = " The error returned by [`try_lookup_or_insert_default()`](crate::Graph::try_lookup_or_insert_default())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Lookup (# [from] gix_object :: find :: existing_iter :: Error) , # [error (transparent)] ToOwned (# [from] to_owned :: Error) , } } }
};
}
