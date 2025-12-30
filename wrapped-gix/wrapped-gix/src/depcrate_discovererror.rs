// Generated macro for Error (enum)
macro_rules! Depcrate_discoverError {
() => {
// Module: crate::discover
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`crate::discover()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Discover (# [from] upwards :: Error) , # [error (transparent)] Open (# [from] crate :: open :: Error) , }
};
}
