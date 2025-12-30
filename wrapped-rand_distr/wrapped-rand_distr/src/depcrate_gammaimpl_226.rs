// Generated macro for impl_226 (impl)
macro_rules! Depcrate_gammaimpl_226 {
() => {
// Module: crate::gamma
// Provides: {"impl_226"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: ShapeTooSmall => "shape is not positive in gamma distribution" , Error :: ScaleTooSmall => "scale is not positive in gamma distribution" , Error :: ScaleTooLarge => "scale is infinity in gamma distribution" , }) } }
};
}
