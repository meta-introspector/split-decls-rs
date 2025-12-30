// Generated macro for impl_254 (impl)
macro_rules! Depcrate_non_zeroimpl_254 {
() => {
// Module: crate::non_zero
// Provides: {"impl_254"}
// Dependencies: {}
impl < T > fmt :: UpperHex for NonZero < T > where T : fmt :: UpperHex + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (& self . 0 , f) } }
};
}
