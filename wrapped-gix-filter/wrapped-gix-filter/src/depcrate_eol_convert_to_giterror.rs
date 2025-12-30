// Generated macro for Error (enum)
macro_rules! Depcrate_eol_convert_to_gitError {
() => {
// Module: crate::eol::convert_to_git
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [convert_to_git()][super::convert_to_git()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("{msg} in '{}'" , path . display ())] RoundTrip { msg : & 'static str , path : PathBuf } , # [error ("Could not obtain index object to check line endings for")] FetchObjectFromIndex (# [source] Box < dyn std :: error :: Error + Send + Sync + 'static >) , # [error ("Could not allocate buffer")] OutOfMemory (# [from] std :: collections :: TryReserveError) , }
};
}
