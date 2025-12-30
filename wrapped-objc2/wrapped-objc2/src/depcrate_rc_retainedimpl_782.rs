// Generated macro for impl_782 (impl)
macro_rules! Depcrate_rc_retainedimpl_782 {
() => {
// Module: crate::rc::retained
// Provides: {"impl_782"}
// Dependencies: {}
impl < T : ? Sized > fmt :: Pointer for Retained < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Pointer :: fmt (& self . ptr . as_ptr () , f) } }
};
}
