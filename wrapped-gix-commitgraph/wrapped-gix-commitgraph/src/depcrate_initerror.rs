// Generated macro for Error (enum)
macro_rules! Depcrate_initError {
() => {
// Module: crate::init
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by initializations functions like [`Graph::at()`]."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("{}" , . path . display ())] File { # [source] err : file :: Error , path : PathBuf , } , # [error ("Commit-graph files mismatch: '{}' uses hash {hash1:?}, but '{}' uses hash {hash2:?}" , . path1 . display () , . path2 . display ())] HashVersionMismatch { path1 : PathBuf , hash1 : gix_hash :: Kind , path2 : PathBuf , hash2 : gix_hash :: Kind , } , # [error ("Did not find any files that look like commit graphs at '{}'" , . 0 . display ())] InvalidPath (PathBuf) , # [error ("Could not open commit-graph file at '{}'" , . path . display ())] Io { # [source] err : std :: io :: Error , path : PathBuf , } , # [error ("Commit-graph files contain {0} commits altogether, but only {MAX_COMMITS} commits are allowed")] TooManyCommits (u64) , }
};
}
