// Generated macro for impl_126 (impl)
macro_rules! Depcrate_boxedimpl_126 {
() => {
// Module: crate::boxed
// Provides: {"impl_126"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : Clone , A : Allocator + Clone > Clone for Box < T , A > { # [doc = " Returns a new box with a `clone()` of this box's contents."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let x = Box::new(5);"] # [doc = " let y = x.clone();"] # [doc = ""] # [doc = " // The value is the same"] # [doc = " assert_eq!(x, y);"] # [doc = ""] # [doc = " // But they are unique objects"] # [doc = " assert_ne!(&*x as *const i32, &*y as *const i32);"] # [doc = " ```"] # [inline] fn clone (& self) -> Self { let mut boxed = Self :: new_uninit_in (self . 1 . clone ()) ; unsafe { (* * self) . clone_to_uninit (boxed . as_mut_ptr () . cast ()) ; boxed . assume_init () } } # [doc = " Copies `source`'s contents into `self` without creating a new allocation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let x = Box::new(5);"] # [doc = " let mut y = Box::new(10);"] # [doc = " let yp: *const i32 = &*y;"] # [doc = ""] # [doc = " y.clone_from(&x);"] # [doc = ""] # [doc = " // The value is the same"] # [doc = " assert_eq!(x, y);"] # [doc = ""] # [doc = " // And no allocation occurred"] # [doc = " assert_eq!(yp, &*y);"] # [doc = " ```"] # [inline] fn clone_from (& mut self , source : & Self) { (* * self) . clone_from (& (* * source)) ; } }
};
}
