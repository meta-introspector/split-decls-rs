// Generated macro for impl_1314 (impl)
macro_rules! Depcrate_remote_connection_ref_mapimpl_1314 {
() => {
// Module: crate::remote::connection::ref_map
// Provides: {"impl_1314"}
// Dependencies: {}
impl gix_protocol :: transport :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Transport (err) => err . is_spurious () , Error :: Handshake (err) => err . is_spurious () , _ => false , } } }
};
}
