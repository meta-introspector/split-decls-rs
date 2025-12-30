// Generated macro for impl_1318 (impl)
macro_rules! Depcrate_rcimpl_1318 {
() => {
// Module: crate::rc
// Provides: {"impl_1318"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > Deref for UniqueRc < T , A > { type Target = T ; fn deref (& self) -> & T { unsafe { & self . ptr . as_ref () . value } } }
};
}
