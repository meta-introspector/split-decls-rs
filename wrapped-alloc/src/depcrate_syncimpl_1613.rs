// Generated macro for impl_1613 (impl)
macro_rules! Depcrate_syncimpl_1613 {
() => {
// Module: crate::sync
// Provides: {"impl_1613"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized , A : Allocator > fmt :: Pointer for Arc < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Pointer :: fmt (& (& raw const * * self) , f) } }
};
}
