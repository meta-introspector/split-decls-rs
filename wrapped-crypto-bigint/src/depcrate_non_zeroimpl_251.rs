// Generated macro for impl_251 (impl)
macro_rules! Depcrate_non_zeroimpl_251 {
() => {
// Module: crate::non_zero
// Provides: {"impl_251"}
// Dependencies: {}
impl < T > fmt :: Binary for NonZero < T > where T : fmt :: Binary + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Binary :: fmt (& self . 0 , f) } }
};
}
