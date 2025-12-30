// Generated macro for impl_1670 (impl)
macro_rules! Depcrate_syncimpl_1670 {
() => {
// Module: crate::sync
// Provides: {"impl_1670"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > Deref for UniqueArc < T , A > { type Target = T ; fn deref (& self) -> & T { unsafe { & self . ptr . as_ref () . data } } }
};
}
