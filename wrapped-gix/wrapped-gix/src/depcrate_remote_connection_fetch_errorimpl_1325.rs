// Generated macro for impl_1325 (impl)
macro_rules! Depcrate_remote_connection_fetch_errorimpl_1325 {
() => {
// Module: crate::remote::connection::fetch::error
// Provides: {"impl_1325"}
// Dependencies: {}
impl gix_protocol :: transport :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Fetch (err) => err . is_spurious () , Error :: Client (err) => err . is_spurious () , _ => false , } } }
};
}
