// Generated macro for impl_1622 (impl)
macro_rules! Depcrate_syncimpl_1622 {
() => {
// Module: crate::sync
// Provides: {"impl_1622"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + Hash , A : Allocator > Hash for Arc < T , A > { fn hash < H : Hasher > (& self , state : & mut H) { (* * self) . hash (state) } }
};
}
