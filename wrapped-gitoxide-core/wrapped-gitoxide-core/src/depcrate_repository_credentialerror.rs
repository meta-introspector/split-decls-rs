// Generated macro for Error (enum)
macro_rules! Depcrate_repository_credentialError {
() => {
// Module: crate::repository::credential
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug , thiserror :: Error)] enum Error { # [error (transparent)] UrlParse (# [from] gix :: url :: parse :: Error) , # [error (transparent)] Configuration (# [from] gix :: config :: credential_helpers :: Error) , # [error (transparent)] Protocol (# [from] gix :: credentials :: protocol :: Error) , # [error (transparent)] ConfigLoad (# [from] gix :: config :: file :: init :: from_paths :: Error) , }
};
}
