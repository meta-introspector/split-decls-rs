// Generated macro for impl_30 (impl)
macro_rules! Depcrate_contextimpl_30 {
() => {
// Module: crate::context
// Provides: {"impl_30"}
// Dependencies: {}
impl < D , E > StdError for ContextError < D , E > where D : Display , E : StdError + 'static , { # [cfg (generic_member_access)] fn provide < 'a > (& 'a self , request : & mut std :: error :: Request < 'a >) { self . error . provide (request) ; } fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (& self . error) } }
};
}
