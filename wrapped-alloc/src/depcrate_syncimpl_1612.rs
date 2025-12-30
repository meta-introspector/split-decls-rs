// Generated macro for impl_1612 (impl)
macro_rules! Depcrate_syncimpl_1612 {
() => {
// Module: crate::sync
// Provides: {"impl_1612"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + fmt :: Debug , A : Allocator > fmt :: Debug for Arc < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
