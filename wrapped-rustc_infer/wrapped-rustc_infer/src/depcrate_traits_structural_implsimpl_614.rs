// Generated macro for impl_614 (impl)
macro_rules! Depcrate_traits_structural_implsimpl_614 {
() => {
// Module: crate::traits::structural_impls
// Provides: {"impl_614"}
// Dependencies: {}
impl < 'tcx , O : fmt :: Debug > fmt :: Debug for traits :: Obligation < 'tcx , O > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if ty :: tls :: with (| tcx | tcx . sess . verbose_internals ()) { write ! (f , "Obligation(predicate={:?}, cause={:?}, param_env={:?}, depth={})" , self . predicate , self . cause , self . param_env , self . recursion_depth) } else { write ! (f , "Obligation(predicate={:?}, depth={})" , self . predicate , self . recursion_depth) } } }
};
}
