// Generated macro for impl_180 (impl)
macro_rules! Depcrate_normalizeimpl_180 {
() => {
// Module: crate::normalize
// Provides: {"impl_180"}
// Dependencies: {}
impl < S : Spec > fmt :: Display for Normalized < '_ , RiStr < S > > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { NormalizedInner :: < S > :: from_input (self . input) . fmt (f) } }
};
}
