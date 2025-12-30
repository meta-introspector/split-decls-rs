// Generated macro for impl_1229 (impl)
macro_rules! Depcrate_rcimpl_1229 {
() => {
// Module: crate::rc
// Provides: {"impl_1229"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized , A : Allocator > Deref for Rc < T , A > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { & self . inner () . value } }
};
}
