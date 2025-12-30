// Generated macro for Error (enum)
macro_rules! Depcrate_store_impls_loose_writeError {
() => {
// Module: crate::store_impls::loose::write
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Returned by the [`gix_object::Write`] trait implementation of [`Store`]"] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("Could not {message} '{path}'")] Io { source : gix_hash :: io :: Error , message : & 'static str , path : PathBuf , } , # [error ("An IO error occurred while writing an object")] IoRaw (# [from] io :: Error) , # [error ("Could not turn temporary file into persisted file at '{target}'")] Persist { source : tempfile :: PersistError , target : PathBuf , } , }
};
}
