// Generated macro for impl_1640 (impl)
macro_rules! Depcrate_syncimpl_1640 {
() => {
// Module: crate::sync
// Provides: {"impl_1640"}
// Dependencies: {}
# [stable (since = "1.5.0" , feature = "smart_ptr_as_ref")] impl < T : ? Sized , A : Allocator > AsRef < T > for Arc < T , A > { fn as_ref (& self) -> & T { & * * self } }
};
}
