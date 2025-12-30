// Generated macro for error (module)
macro_rules! Depcrate_multi_index_writeerror {
() => {
// Module: crate::multi_index::write
// Provides: {"error"}
// Dependencies: {}
mod error { # [doc = " The error returned by [`multi_index::File::write_from_index_paths()`][super::multi_index::File::write_from_index_paths()].."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Io (# [from] gix_hash :: io :: Error) , # [error ("Interrupted")] Interrupted , # [error (transparent)] OpenIndex (# [from] crate :: index :: init :: Error) , } }
};
}
