// Generated macro for impl_24 (impl)
macro_rules! Depcrate_attrsimpl_24 {
() => {
// Module: crate::attrs
// Provides: {"impl_24"}
// Dependencies: {}
impl fmt :: Display for AttrInput { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { AttrInput :: Literal (lit) => write ! (f , " = {lit}") , AttrInput :: TokenTree (tt) => tt . fmt (f) , } } }
};
}
