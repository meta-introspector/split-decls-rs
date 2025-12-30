// Generated macro for impl_1306 (impl)
macro_rules! Depcrate_rcimpl_1306 {
() => {
// Module: crate::rc
// Provides: {"impl_1306"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > AsRef < T > for UniqueRc < T , A > { fn as_ref (& self) -> & T { & * * self } }
};
}
