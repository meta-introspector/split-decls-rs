// Generated macro for impl_98 (impl)
macro_rules! Depcrate_ext_impls_impl_const_defaultimpl_98 {
() => {
// Module: crate::ext_impls::impl_const_default
// Provides: {"impl_98"}
// Dependencies: {}
impl < T , U : ConstDefault > ConstDefault for GenericArrayImplEven < T , U > { const DEFAULT : Self = Self { parents : [U :: DEFAULT ; 2] , _marker : core :: marker :: PhantomData , } ; }
};
}
