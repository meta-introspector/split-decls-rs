// Generated macro for impl_253 (impl)
macro_rules! Depcrate_non_zeroimpl_253 {
() => {
// Module: crate::non_zero
// Provides: {"impl_253"}
// Dependencies: {}
impl < T > fmt :: LowerHex for NonZero < T > where T : fmt :: LowerHex + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . 0 , f) } }
};
}
