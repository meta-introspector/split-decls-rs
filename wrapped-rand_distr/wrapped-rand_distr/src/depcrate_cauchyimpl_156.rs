// Generated macro for impl_156 (impl)
macro_rules! Depcrate_cauchyimpl_156 {
() => {
// Module: crate::cauchy
// Provides: {"impl_156"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: ScaleTooSmall => "scale is not positive in Cauchy distribution" , }) } }
};
}
