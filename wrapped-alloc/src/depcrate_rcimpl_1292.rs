// Generated macro for impl_1292 (impl)
macro_rules! Depcrate_rcimpl_1292 {
() => {
// Module: crate::rc
// Provides: {"impl_1292"}
// Dependencies: {}
# [stable (since = "1.5.0" , feature = "smart_ptr_as_ref")] impl < T : ? Sized , A : Allocator > AsRef < T > for Rc < T , A > { fn as_ref (& self) -> & T { & * * self } }
};
}
