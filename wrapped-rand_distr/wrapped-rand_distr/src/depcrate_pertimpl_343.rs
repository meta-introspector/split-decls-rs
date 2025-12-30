// Generated macro for impl_343 (impl)
macro_rules! Depcrate_pertimpl_343 {
() => {
// Module: crate::pert
// Provides: {"impl_343"}
// Dependencies: {}
impl fmt :: Display for PertError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { PertError :: RangeTooSmall => "requirement min < max is not met in PERT distribution" , PertError :: ModeRange => "mode is outside [min, max] in PERT distribution" , PertError :: ShapeTooSmall => "shape < 0 or is NaN in PERT distribution" , }) } }
};
}
