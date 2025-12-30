// Generated macro for Error (enum)
macro_rules! Depcrate_parseError {
() => {
// Module: crate::parse
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by the [`parse()`][crate::parse()] function."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Empty refspecs are invalid")] Empty , # [error ("Negative refspecs cannot have destinations as they exclude sources")] NegativeWithDestination , # [error ("Negative specs must not be empty")] NegativeEmpty , # [error ("Negative specs must be object hashes")] NegativeObjectHash , # [error ("Negative specs must be full ref names, starting with \"refs/\"")] NegativePartialName , # [error ("Negative glob patterns are not allowed")] NegativeGlobPattern , # [error ("Fetch destinations must be ref-names, like 'HEAD:refs/heads/branch'")] InvalidFetchDestination , # [error ("Cannot push into an empty destination")] PushToEmpty , # [error ("glob patterns may only involved a single '*' character, found {pattern:?}")] PatternUnsupported { pattern : bstr :: BString } , # [error ("Both sides of the specification need a pattern, like 'a/*:b/*'")] PatternUnbalanced , # [error (transparent)] ReferenceName (# [from] gix_validate :: reference :: name :: Error) , # [error (transparent)] RevSpec (# [from] gix_revision :: spec :: parse :: Error) , }
};
}
