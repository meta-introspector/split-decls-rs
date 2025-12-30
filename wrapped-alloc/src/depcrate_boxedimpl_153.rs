// Generated macro for impl_153 (impl)
macro_rules! Depcrate_boxedimpl_153 {
() => {
// Module: crate::boxed
// Provides: {"impl_153"}
// Dependencies: {}
# [stable (since = "1.5.0" , feature = "smart_ptr_as_ref")] impl < T : ? Sized , A : Allocator > AsRef < T > for Box < T , A > { fn as_ref (& self) -> & T { & * * self } }
};
}
