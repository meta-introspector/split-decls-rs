// Generated macro for Error (enum)
macro_rules! Depcrate_createError {
() => {
// Module: crate::create
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error used in [`into()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not obtain the current directory")] CurrentDir (# [from] std :: io :: Error) , # [error ("Could not open data at '{}'" , . path . display ())] IoOpen { source : std :: io :: Error , path : PathBuf } , # [error ("Could not write data at '{}'" , . path . display ())] IoWrite { source : std :: io :: Error , path : PathBuf } , # [error ("Refusing to initialize the existing '{}' directory" , . path . display ())] DirectoryExists { path : PathBuf } , # [error ("Refusing to initialize the non-empty directory as '{}'" , . path . display ())] DirectoryNotEmpty { path : PathBuf } , # [error ("Could not create directory at '{}'" , . path . display ())] CreateDirectory { source : std :: io :: Error , path : PathBuf } , }
};
}
