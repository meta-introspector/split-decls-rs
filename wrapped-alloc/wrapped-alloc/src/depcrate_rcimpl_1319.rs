// Generated macro for impl_1319 (impl)
macro_rules! Depcrate_rcimpl_1319 {
() => {
// Module: crate::rc
// Provides: {"impl_1319"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > DerefMut for UniqueRc < T , A > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut (* self . ptr . as_ptr ()) . value } } }
};
}
