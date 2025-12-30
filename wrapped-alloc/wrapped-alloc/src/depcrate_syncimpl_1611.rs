// Generated macro for impl_1611 (impl)
macro_rules! Depcrate_syncimpl_1611 {
() => {
// Module: crate::sync
// Provides: {"impl_1611"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + fmt :: Display , A : Allocator > fmt :: Display for Arc < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& * * self , f) } }
};
}
