// Generated macro for impl_154 (impl)
macro_rules! Depcrate_boxedimpl_154 {
() => {
// Module: crate::boxed
// Provides: {"impl_154"}
// Dependencies: {}
# [stable (since = "1.5.0" , feature = "smart_ptr_as_ref")] impl < T : ? Sized , A : Allocator > AsMut < T > for Box < T , A > { fn as_mut (& mut self) -> & mut T { & mut * * self } }
};
}
