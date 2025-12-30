// Generated macro for impl_181 (impl)
macro_rules! Depcrate_normalizeimpl_181 {
() => {
// Module: crate::normalize
// Provides: {"impl_181"}
// Dependencies: {}
impl < S : Spec > fmt :: Display for Normalized < '_ , RiAbsoluteStr < S > > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { NormalizedInner :: < S > :: from_input (self . input) . fmt (f) } }
};
}
