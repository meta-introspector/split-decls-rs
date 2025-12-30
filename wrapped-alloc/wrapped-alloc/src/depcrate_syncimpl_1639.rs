// Generated macro for impl_1639 (impl)
macro_rules! Depcrate_syncimpl_1639 {
() => {
// Module: crate::sync
// Provides: {"impl_1639"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized , A : Allocator > borrow :: Borrow < T > for Arc < T , A > { fn borrow (& self) -> & T { & * * self } }
};
}
