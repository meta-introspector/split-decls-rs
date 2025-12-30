// Generated macro for Error (enum)
macro_rules! Depcrate_file_writeError {
() => {
// Module: crate::file::write
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error produced by [`File::write()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Io (# [from] gix_hash :: io :: Error) , # [error ("Could not acquire lock for index file")] AcquireLock (# [from] gix_lock :: acquire :: Error) , # [error ("Could not commit lock for index file")] CommitLock (# [from] gix_lock :: commit :: Error < gix_lock :: File >) , }
};
}
