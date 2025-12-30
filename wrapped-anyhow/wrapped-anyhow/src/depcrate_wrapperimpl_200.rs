// Generated macro for impl_200 (impl)
macro_rules! Depcrate_wrapperimpl_200 {
() => {
// Module: crate::wrapper
// Provides: {"impl_200"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl StdError for BoxedError { fn source (& self) -> Option < & (dyn StdError + 'static) > { self . 0 . source () } # [cfg (error_generic_member_access)] fn provide < 'a > (& 'a self , request : & mut Request < 'a >) { nightly :: provide (& * self . 0 , request) ; } }
};
}
