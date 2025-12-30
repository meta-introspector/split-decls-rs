// Generated macro for impl_44 (impl)
macro_rules! Depcrate_fetch_responseimpl_44 {
() => {
// Module: crate::fetch::response
// Provides: {"impl_44"}
// Dependencies: {}
impl gix_transport :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Io (err) => err . is_spurious () , Error :: Transport (err) => err . is_spurious () , _ => false , } } }
};
}
