// Generated macro for impl_1665 (impl)
macro_rules! Depcrate_syncimpl_1665 {
() => {
// Module: crate::sync
// Provides: {"impl_1665"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized + Hash , A : Allocator > Hash for UniqueArc < T , A > { fn hash < H : Hasher > (& self , state : & mut H) { (* * self) . hash (state) ; } }
};
}
