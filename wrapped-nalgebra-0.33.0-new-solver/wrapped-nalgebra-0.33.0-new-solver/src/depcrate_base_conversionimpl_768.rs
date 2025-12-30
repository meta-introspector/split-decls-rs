// Generated macro for impl_768 (impl)
macro_rules! Depcrate_base_conversionimpl_768 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_768"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < SVector < T , D > > for [T ; D] { # [inline] fn from (vec : SVector < T , D >) -> Self { vec . data . 0 [0] . clone () } }
};
}
