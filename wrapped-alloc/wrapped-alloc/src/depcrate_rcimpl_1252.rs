// Generated macro for impl_1252 (impl)
macro_rules! Depcrate_rcimpl_1252 {
() => {
// Module: crate::rc
// Provides: {"impl_1252"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + Hash , A : Allocator > Hash for Rc < T , A > { fn hash < H : Hasher > (& self , state : & mut H) { (* * self) . hash (state) ; } }
};
}
