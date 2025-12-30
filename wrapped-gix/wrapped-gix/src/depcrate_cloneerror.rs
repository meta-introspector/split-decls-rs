// Generated macro for Error (enum)
macro_rules! Depcrate_cloneError {
() => {
// Module: crate::clone
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`PrepareFetch::new()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Init (# [from] crate :: init :: Error) , # [error (transparent)] CommitterOrFallback (# [from] crate :: config :: time :: Error) , # [error (transparent)] UrlParse (# [from] gix_url :: parse :: Error) , # [error ("Failed to turn a the relative file url \"{}\" into an absolute one" , url . to_bstring ())] CanonicalizeUrl { url : gix_url :: Url , source : gix_path :: realpath :: Error , } , }
};
}
