// Generated macro for impl_1672 (impl)
macro_rules! Depcrate_syncimpl_1672 {
() => {
// Module: crate::sync
// Provides: {"impl_1672"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > DerefMut for UniqueArc < T , A > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut (* self . ptr . as_ptr ()) . data } } }
};
}
