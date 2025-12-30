// Generated macro for impl_1658 (impl)
macro_rules! Depcrate_syncimpl_1658 {
() => {
// Module: crate::sync
// Provides: {"impl_1658"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > AsRef < T > for UniqueArc < T , A > { fn as_ref (& self) -> & T { & * * self } }
};
}
