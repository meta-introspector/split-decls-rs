// Generated macro for error (module)
macro_rules! Depcrate_ls_refserror {
() => {
// Module: crate::ls_refs
// Provides: {"error"}
// Dependencies: {}
# [cfg (any (feature = "blocking-client" , feature = "async-client"))] mod error { use crate :: handshake :: refs :: parse ; # [doc = " The error returned by [`ls_refs()`][crate::ls_refs()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Io (# [from] std :: io :: Error) , # [error (transparent)] Transport (# [from] gix_transport :: client :: Error) , # [error (transparent)] Parse (# [from] parse :: Error) , # [error (transparent)] ArgumentValidation (# [from] crate :: command :: validate_argument_prefixes :: Error) , } impl gix_transport :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Io (err) => err . is_spurious () , Error :: Transport (err) => err . is_spurious () , _ => false , } } } }
};
}
