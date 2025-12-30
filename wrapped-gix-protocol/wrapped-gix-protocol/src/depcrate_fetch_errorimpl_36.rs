// Generated macro for impl_36 (impl)
macro_rules! Depcrate_fetch_errorimpl_36 {
() => {
// Module: crate::fetch::error
// Provides: {"impl_36"}
// Dependencies: {}
impl crate :: transport :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: FetchResponse (err) => err . is_spurious () , Error :: Client (err) => err . is_spurious () , _ => false , } } }
};
}
