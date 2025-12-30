// Generated macro for Error (enum)
macro_rules! Depcrate_realpathError {
() => {
// Module: crate::realpath
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`realpath()`][super::realpath()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The maximum allowed number {} of symlinks in path is exceeded" , . max_symlinks)] MaxSymlinksExceeded { max_symlinks : u8 } , # [error ("Cannot resolve symlinks in path with more than {max_symlink_checks} components (takes too long)")] ExcessiveComponentCount { max_symlink_checks : usize } , # [error (transparent)] ReadLink (std :: io :: Error) , # [error (transparent)] CurrentWorkingDir (std :: io :: Error) , # [error ("Empty is not a valid path")] EmptyPath , # [error ("Ran out of path components while following parent component '..'")] MissingParent , }
};
}
