// Generated macro for negotiate (module)
macro_rules! Depcrate_remote_fetchnegotiate {
() => {
// Module: crate::remote::fetch
// Provides: {"negotiate"}
// Dependencies: {}
# [doc = ""] pub mod negotiate { # [cfg (feature = "credentials")] pub use gix_negotiate :: Algorithm ; # [cfg (any (feature = "blocking-network-client" , feature = "async-network-client"))] pub use gix_protocol :: fetch :: negotiate :: Error ; }
};
}
