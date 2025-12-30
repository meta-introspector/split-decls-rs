// Generated macro for Error (enum)
macro_rules! Depcrate_cargo_metadataError {
() => {
// Module: crate::cargo_metadata
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Describes how this module can fail"] # [derive (Debug , thiserror :: Error)] pub enum Error { # [error ("I/O Error: {0:?}")] Io (# [from] std :: io :: Error) , # [error ("Failed get output from cargo-metadata: {0:?}")] GettingMetadata (# [from] cargo_metadata :: Error) , # [error ("Bad path {0:?} whilst scraping files")] Scraping (PathBuf) , }
};
}
