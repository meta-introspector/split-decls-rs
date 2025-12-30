// Generated macro for impl_100 (impl)
macro_rules! Depcrate_ext_impls_impl_const_defaultimpl_100 {
() => {
// Module: crate::ext_impls::impl_const_default
// Provides: {"impl_100"}
// Dependencies: {}
impl < T , U : ArrayLength > ConstDefault for GenericArray < T , U > where U :: ArrayType < T > : ConstDefault , { const DEFAULT : Self = Self { data : ConstDefault :: DEFAULT , } ; }
};
}
