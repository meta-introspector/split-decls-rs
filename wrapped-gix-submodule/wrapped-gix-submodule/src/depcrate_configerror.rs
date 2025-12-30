// Generated macro for Error (struct)
macro_rules! Depcrate_configError {
() => {
// Module: crate::config
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [File::fetch_recurse()](crate::File::fetch_recurse) and [File::ignore()](crate::File::ignore)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] # [error ("The '{field}' field of submodule '{submodule}' was invalid: '{actual}'")] pub struct Error { pub field : & 'static str , pub submodule : BString , pub actual : BString , }
};
}
