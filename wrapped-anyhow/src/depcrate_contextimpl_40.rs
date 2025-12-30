// Generated macro for impl_40 (impl)
macro_rules! Depcrate_contextimpl_40 {
() => {
// Module: crate::context
// Provides: {"impl_40"}
// Dependencies: {}
impl < C , E > StdError for ContextError < C , E > where C : Display , E : StdError + 'static , { fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (& self . error) } # [cfg (error_generic_member_access)] fn provide < 'a > (& 'a self , request : & mut Request < 'a >) { nightly :: provide (& self . error , request) ; } }
};
}
