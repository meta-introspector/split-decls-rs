// Generated macro for impl_1584 (impl)
macro_rules! Depcrate_syncimpl_1584 {
() => {
// Module: crate::sync
// Provides: {"impl_1584"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized , A : Allocator > Deref for Arc < T , A > { type Target = T ; # [inline] fn deref (& self) -> & T { & self . inner () . data } }
};
}
