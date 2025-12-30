// Generated macro for impl_40 (impl)
macro_rules! Depcrate_atomicimpl_40 {
() => {
// Module: crate::atomic
// Provides: {"impl_40"}
// Dependencies: {}
impl < T : ? Sized + Pointable > fmt :: Pointer for Atomic < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let data = self . data . load (Ordering :: SeqCst) ; let (raw , _) = decompose_tag :: < T > (data) ; fmt :: Pointer :: fmt (& (unsafe { T :: deref (raw) as * const _ }) , f) } }
};
}
