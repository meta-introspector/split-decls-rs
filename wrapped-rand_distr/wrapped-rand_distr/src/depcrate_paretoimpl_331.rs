// Generated macro for impl_331 (impl)
macro_rules! Depcrate_paretoimpl_331 {
() => {
// Module: crate::pareto
// Provides: {"impl_331"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: ScaleTooSmall => "scale is not positive in Pareto distribution" , Error :: ShapeTooSmall => "shape is not positive in Pareto distribution" , }) } }
};
}
