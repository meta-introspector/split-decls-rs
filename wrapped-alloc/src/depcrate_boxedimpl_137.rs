// Generated macro for impl_137 (impl)
macro_rules! Depcrate_boxedimpl_137 {
() => {
// Module: crate::boxed
// Provides: {"impl_137"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized , A : Allocator > fmt :: Pointer for Box < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let ptr : * const T = & * * self ; fmt :: Pointer :: fmt (& ptr , f) } }
};
}
