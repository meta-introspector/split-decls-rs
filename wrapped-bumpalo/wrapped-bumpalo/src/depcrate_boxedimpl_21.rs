// Generated macro for impl_21 (impl)
macro_rules! Depcrate_boxedimpl_21 {
() => {
// Module: crate::boxed
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a , T : ? Sized > fmt :: Pointer for Box < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let ptr : * const T = & * * self ; fmt :: Pointer :: fmt (& ptr , f) } }
};
}
