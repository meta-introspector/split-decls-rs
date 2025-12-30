// Generated macro for impl_133 (impl)
macro_rules! Depcrate_boxedimpl_133 {
() => {
// Module: crate::boxed
// Provides: {"impl_133"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + Hash , A : Allocator > Hash for Box < T , A > { fn hash < H : Hasher > (& self , state : & mut H) { (* * self) . hash (state) ; } }
};
}
