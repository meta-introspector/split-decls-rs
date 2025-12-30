// Generated macro for impl_1313 (impl)
macro_rules! Depcrate_rcimpl_1313 {
() => {
// Module: crate::rc
// Provides: {"impl_1313"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized + Hash , A : Allocator > Hash for UniqueRc < T , A > { fn hash < H : Hasher > (& self , state : & mut H) { (* * self) . hash (state) ; } }
};
}
