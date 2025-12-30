// Generated macro for impl_127 (impl)
macro_rules! Depcrate_boxedimpl_127 {
() => {
// Module: crate::boxed
// Provides: {"impl_127"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "box_slice_clone" , since = "1.3.0")] impl < T : Clone , A : Allocator + Clone > Clone for Box < [T] , A > { fn clone (& self) -> Self { let alloc = Box :: allocator (self) . clone () ; self . to_vec_in (alloc) . into_boxed_slice () } # [doc = " Copies `source`'s contents into `self` without creating a new allocation,"] # [doc = " so long as the two are of the same length."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let x = Box::new([5, 6, 7]);"] # [doc = " let mut y = Box::new([8, 9, 10]);"] # [doc = " let yp: *const [i32] = &*y;"] # [doc = ""] # [doc = " y.clone_from(&x);"] # [doc = ""] # [doc = " // The value is the same"] # [doc = " assert_eq!(x, y);"] # [doc = ""] # [doc = " // And no allocation occurred"] # [doc = " assert_eq!(yp, &*y);"] # [doc = " ```"] fn clone_from (& mut self , source : & Self) { if self . len () == source . len () { self . clone_from_slice (& source) ; } else { * self = source . clone () ; } } }
};
}
