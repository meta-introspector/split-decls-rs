// Generated macro for impl_88 (impl)
macro_rules! Depcrate_boxedimpl_88 {
() => {
// Module: crate::boxed
// Provides: {"impl_88"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator > fmt :: Pointer for Box < T , A > { # [inline (always)] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let ptr : * const T = & * * self ; fmt :: Pointer :: fmt (& ptr , f) } }
};
}
