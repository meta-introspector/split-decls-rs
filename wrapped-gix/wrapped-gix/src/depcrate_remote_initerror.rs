// Generated macro for error (module)
macro_rules! Depcrate_remote_initerror {
() => {
// Module: crate::remote::init
// Provides: {"error"}
// Dependencies: {}
mod error { use crate :: bstr :: BString ; # [doc = " The error returned by [`Repository::remote_at(…)`][crate::Repository::remote_at()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Url (# [from] gix_url :: parse :: Error) , # [error ("The rewritten {kind} url {rewritten_url:?} failed to parse")] RewrittenUrlInvalid { kind : & 'static str , rewritten_url : BString , source : gix_url :: parse :: Error , } , } }
};
}
