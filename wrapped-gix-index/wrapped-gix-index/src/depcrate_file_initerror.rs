// Generated macro for error (module)
macro_rules! Depcrate_file_initerror {
() => {
// Module: crate::file::init
// Provides: {"error"}
// Dependencies: {}
mod error { # [doc = " The error returned by [File::at()][super::File::at()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("An IO error occurred while opening the index")] Io (# [from] std :: io :: Error) , # [error (transparent)] Decode (# [from] crate :: decode :: Error) , # [error (transparent)] LinkExtension (# [from] crate :: extension :: link :: decode :: Error) , } }
};
}
