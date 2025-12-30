// Generated macro for impl_372 (impl)
macro_rules! Depcrate_expand_autodiff_attrsimpl_372 {
() => {
// Module: crate::expand::autodiff_attrs
// Provides: {"impl_372"}
// Dependencies: {}
impl fmt :: Display for AutoDiffItem { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Differentiating {} -> {}" , self . source , self . target) ? ; write ! (f , " with attributes: {:?}" , self . attrs) } }
};
}
