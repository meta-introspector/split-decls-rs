// Generated macro for impl_110 (impl)
macro_rules! Depcrate_errorimpl_110 {
() => {
// Module: crate::error
// Provides: {"impl_110"}
// Dependencies: {}
impl < E > StdError for ErrorImpl < E > where E : StdError , { fn source (& self) -> Option < & (dyn StdError + 'static) > { unsafe { ErrorImpl :: error (self . erase ()) . source () } } # [cfg (error_generic_member_access)] fn provide < 'a > (& 'a self , request : & mut Request < 'a >) { unsafe { ErrorImpl :: provide (self . erase () , request) } } }
};
}
