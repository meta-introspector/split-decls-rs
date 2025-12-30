// Generated macro for impl_364 (impl)
macro_rules! Depcrate_expand_autodiff_attrsimpl_364 {
() => {
// Module: crate::expand::autodiff_attrs
// Provides: {"impl_364"}
// Dependencies: {}
impl Display for DiffMode { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { DiffMode :: Error => write ! (f , "Error") , DiffMode :: Source => write ! (f , "Source") , DiffMode :: Forward => write ! (f , "Forward") , DiffMode :: Reverse => write ! (f , "Reverse") , } } }
};
}
