// Generated macro for impl_78 (impl)
macro_rules! Depcrate_atomicimpl_78 {
() => {
// Module: crate::atomic
// Provides: {"impl_78"}
// Dependencies: {}
impl < T : ? Sized + Pointable > fmt :: Pointer for Shared < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Pointer :: fmt (& (unsafe { self . deref () as * const _ }) , f) } }
};
}
