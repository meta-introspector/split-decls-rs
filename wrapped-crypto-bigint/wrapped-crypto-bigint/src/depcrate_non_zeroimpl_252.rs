// Generated macro for impl_252 (impl)
macro_rules! Depcrate_non_zeroimpl_252 {
() => {
// Module: crate::non_zero
// Provides: {"impl_252"}
// Dependencies: {}
impl < T > fmt :: Octal for NonZero < T > where T : fmt :: Octal + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Octal :: fmt (& self . 0 , f) } }
};
}
