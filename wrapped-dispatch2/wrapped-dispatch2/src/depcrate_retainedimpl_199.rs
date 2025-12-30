// Generated macro for impl_199 (impl)
macro_rules! Depcrate_retainedimpl_199 {
() => {
// Module: crate::retained
// Provides: {"impl_199"}
// Dependencies: {}
impl < T : ? Sized > fmt :: Pointer for DispatchRetained < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Pointer :: fmt (& self . ptr . as_ptr () , f) } }
};
}
