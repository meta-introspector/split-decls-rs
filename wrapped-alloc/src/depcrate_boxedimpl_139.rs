// Generated macro for impl_139 (impl)
macro_rules! Depcrate_boxedimpl_139 {
() => {
// Module: crate::boxed
// Provides: {"impl_139"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized , A : Allocator > DerefMut for Box < T , A > { fn deref_mut (& mut self) -> & mut T { & mut * * self } }
};
}
