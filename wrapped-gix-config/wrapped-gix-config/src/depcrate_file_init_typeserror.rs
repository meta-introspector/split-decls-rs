// Generated macro for Error (enum)
macro_rules! Depcrate_file_init_typesError {
() => {
// Module: crate::file::init::types
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`File::from_bytes_no_includes()`][crate::File::from_bytes_no_includes()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Parse (# [from] parse :: Error) , # [error (transparent)] Interpolate (# [from] interpolate :: Error) , # [error (transparent)] Includes (# [from] init :: includes :: Error) , }
};
}
