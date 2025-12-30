// Generated macro for impl_645 (impl)
macro_rules! Depcrate_traitsimpl_645 {
() => {
// Module: crate::traits
// Provides: {"impl_645"}
// Dependencies: {}
impl < 'tcx , T : PartialEq > PartialEq < Obligation < 'tcx , T > > for Obligation < 'tcx , T > { # [inline] fn eq (& self , other : & Obligation < 'tcx , T >) -> bool { self . param_env == other . param_env && self . predicate == other . predicate } }
};
}
