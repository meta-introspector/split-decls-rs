// Generated macro for impl_99 (impl)
macro_rules! Depcrate_ext_impls_impl_const_defaultimpl_99 {
() => {
// Module: crate::ext_impls::impl_const_default
// Provides: {"impl_99"}
// Dependencies: {}
impl < T : ConstDefault , U : ConstDefault > ConstDefault for GenericArrayImplOdd < T , U > { const DEFAULT : Self = Self { parents : [U :: DEFAULT ; 2] , data : T :: DEFAULT , } ; }
};
}
