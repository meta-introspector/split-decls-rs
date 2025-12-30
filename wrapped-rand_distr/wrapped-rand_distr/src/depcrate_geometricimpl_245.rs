// Generated macro for impl_245 (impl)
macro_rules! Depcrate_geometricimpl_245 {
() => {
// Module: crate::geometric
// Provides: {"impl_245"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: InvalidProbability => { "p is NaN or outside the interval [0, 1] in geometric distribution" } }) } }
};
}
