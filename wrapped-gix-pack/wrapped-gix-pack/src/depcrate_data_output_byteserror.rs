// Generated macro for Error (enum)
macro_rules! Depcrate_data_output_bytesError {
() => {
// Module: crate::data::output::bytes
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by `next()` in the [`FromEntriesIter`] iterator."] # [allow (missing_docs)] # [derive (Debug , thiserror :: Error)] pub enum Error < E > where E : std :: error :: Error + 'static , { # [error (transparent)] Io (# [from] gix_hash :: io :: Error) , # [error (transparent)] Input (E) , }
};
}
