// Generated macro for impl_74 (impl)
macro_rules! Depcrate_errorimpl_74 {
() => {
// Module: crate::error
// Provides: {"impl_74"}
// Dependencies: {}
impl < E > StdError for ErrorImpl < E > where E : StdError , { # [cfg (generic_member_access)] fn provide < 'a > (& 'a self , request : & mut std :: error :: Request < 'a >) { self . _object . provide (request) } fn source (& self) -> Option < & (dyn StdError + 'static) > { ErrorImpl :: < () > :: error (self . erase ()) . source () } }
};
}
