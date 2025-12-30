// Generated macro for impl_138 (impl)
macro_rules! Depcrate_boxedimpl_138 {
() => {
// Module: crate::boxed
// Provides: {"impl_138"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized , A : Allocator > Deref for Box < T , A > { type Target = T ; fn deref (& self) -> & T { & * * self } }
};
}
