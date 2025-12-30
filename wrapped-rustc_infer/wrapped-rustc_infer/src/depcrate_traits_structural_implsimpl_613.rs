// Generated macro for impl_613 (impl)
macro_rules! Depcrate_traits_structural_implsimpl_613 {
() => {
// Module: crate::traits::structural_impls
// Provides: {"impl_613"}
// Dependencies: {}
impl < 'tcx , T : fmt :: Debug > fmt :: Debug for Normalized < 'tcx , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Normalized({:?}, {:?})" , self . value , self . obligations) } }
};
}
