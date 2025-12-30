// Generated macro for Error (enum)
macro_rules! Depcrate_expand_pathError {
() => {
// Module: crate::expand_path
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error used by [`parse()`], [`with()`] and [`expand_path()`](crate::expand_path())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("UTF8 conversion on non-unix system failed for path: {path:?}")] IllformedUtf8 { path : BString } , # [error ("Home directory could not be obtained for {}" , match user { Some (user) => format ! ("user '{user}'") , None => "current user" . into () })] MissingHome { user : Option < BString > } , }
};
}
