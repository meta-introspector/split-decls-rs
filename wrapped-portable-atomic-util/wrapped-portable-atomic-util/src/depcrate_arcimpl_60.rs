// Generated macro for impl_60 (impl)
macro_rules! Depcrate_arcimpl_60 {
() => {
// Module: crate::arc
// Provides: {"impl_60"}
// Dependencies: {}
impl < T : ? Sized > fmt :: Pointer for Arc < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Pointer :: fmt (& (& * * self as * const T) , f) } }
};
}
