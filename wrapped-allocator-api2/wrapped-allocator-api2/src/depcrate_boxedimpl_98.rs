// Generated macro for impl_98 (impl)
macro_rules! Depcrate_boxedimpl_98 {
() => {
// Module: crate::boxed
// Provides: {"impl_98"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : Clone , A : Allocator + Clone > Clone for Box < [T] , A > { # [inline (always)] fn clone (& self) -> Self { let alloc = Box :: allocator (self) . clone () ; let mut vec = Vec :: with_capacity_in (self . len () , alloc) ; vec . extend_from_slice (self) ; vec . into_boxed_slice () } # [inline (always)] fn clone_from (& mut self , other : & Self) { if self . len () == other . len () { self . clone_from_slice (other) ; } else { * self = other . clone () ; } } }
};
}
