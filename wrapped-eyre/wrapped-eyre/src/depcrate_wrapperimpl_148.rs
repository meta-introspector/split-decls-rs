// Generated macro for impl_148 (impl)
macro_rules! Depcrate_wrapperimpl_148 {
() => {
// Module: crate::wrapper
// Provides: {"impl_148"}
// Dependencies: {}
impl StdError for BoxedError { # [cfg (generic_member_access)] fn provide < 'a > (& 'a self , request : & mut std :: error :: Request < 'a >) { self . 0 . provide (request) ; } fn source (& self) -> Option < & (dyn StdError + 'static) > { self . 0 . source () } }
};
}
