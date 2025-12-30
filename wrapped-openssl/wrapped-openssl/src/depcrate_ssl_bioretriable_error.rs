// Generated macro for retriable_error (function)
macro_rules! Depcrate_ssl_bioretriable_error {
() => {
// Module: crate::ssl::bio
// Provides: {"retriable_error"}
// Dependencies: {}
# [allow (clippy :: match_like_matches_macro)] fn retriable_error (err : & io :: Error) -> bool { match err . kind () { io :: ErrorKind :: WouldBlock | io :: ErrorKind :: NotConnected => true , _ => false , } }
};
}
