// Generated macro for impl_250 (impl)
macro_rules! Depcrate_non_zeroimpl_250 {
() => {
// Module: crate::non_zero
// Provides: {"impl_250"}
// Dependencies: {}
impl < T > fmt :: Display for NonZero < T > where T : fmt :: Display + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }
};
}
