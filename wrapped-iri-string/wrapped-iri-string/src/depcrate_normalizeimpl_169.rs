// Generated macro for impl_169 (impl)
macro_rules! Depcrate_normalizeimpl_169 {
() => {
// Module: crate::normalize
// Provides: {"impl_169"}
// Dependencies: {}
impl < S : Spec > fmt :: Debug for NormalizedInner < '_ , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Normalized") . field ("input" , & self . input) . finish () } }
};
}
