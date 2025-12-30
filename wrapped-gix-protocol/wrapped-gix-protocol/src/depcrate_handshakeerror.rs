// Generated macro for error (module)
macro_rules! Depcrate_handshakeerror {
() => {
// Module: crate::handshake
// Provides: {"error"}
// Dependencies: {}
# [cfg (feature = "handshake")] mod error { use bstr :: BString ; use gix_transport :: client ; use crate :: { credentials , handshake :: refs } ; # [doc = " The error returned by [`handshake()`][crate::handshake()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to obtain credentials")] Credentials (# [from] credentials :: protocol :: Error) , # [error ("No credentials were returned at all as if the credential helper isn't functioning unknowingly")] EmptyCredentials , # [error ("Credentials provided for \"{url}\" were not accepted by the remote")] InvalidCredentials { url : BString , source : std :: io :: Error } , # [error (transparent)] Transport (# [from] client :: Error) , # [error ("The transport didn't accept the advertised server version {actual_version:?} and closed the connection client side")] TransportProtocolPolicyViolation { actual_version : gix_transport :: Protocol } , # [error (transparent)] ParseRefs (# [from] refs :: parse :: Error) , } impl gix_transport :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Transport (err) => err . is_spurious () , _ => false , } } } }
};
}
