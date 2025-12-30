// Generated macro for Error (enum)
macro_rules! Depcrate_remote_saveError {
() => {
// Module: crate::remote::save
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`Remote::save_to()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The remote pointing to {} is anonymous and can't be saved." , url . to_bstring ())] NameMissing { url : gix_url :: Url } , }
};
}
