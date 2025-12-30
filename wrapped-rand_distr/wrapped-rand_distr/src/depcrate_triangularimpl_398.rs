// Generated macro for impl_398 (impl)
macro_rules! Depcrate_triangularimpl_398 {
() => {
// Module: crate::triangular
// Provides: {"impl_398"}
// Dependencies: {}
impl fmt :: Display for TriangularError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { TriangularError :: RangeTooSmall => { "requirement min <= max is not met in triangular distribution" } TriangularError :: ModeRange => "mode is outside [min, max] in triangular distribution" , }) } }
};
}
