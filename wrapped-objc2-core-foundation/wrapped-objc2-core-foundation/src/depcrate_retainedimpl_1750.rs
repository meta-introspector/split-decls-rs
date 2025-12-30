// Generated macro for impl_1750 (impl)
macro_rules! Depcrate_retainedimpl_1750 {
() => {
// Module: crate::retained
// Provides: {"impl_1750"}
// Dependencies: {}
impl < T : ? Sized > fmt :: Pointer for CFRetained < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Pointer :: fmt (& self . ptr . as_ptr () , f) } }
};
}
