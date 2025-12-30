// Generated macro for impl_212 (impl)
macro_rules! Depcrate_frechetimpl_212 {
() => {
// Module: crate::frechet
// Provides: {"impl_212"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: LocationNotFinite => "location is not finite in Frechet distribution" , Error :: ScaleNotPositive => "scale is not positive and finite in Frechet distribution" , Error :: ShapeNotPositive => "shape is not positive and finite in Frechet distribution" , }) } }
};
}
