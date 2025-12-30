// Generated macro for impl_1257 (impl)
macro_rules! Depcrate_rcimpl_1257 {
() => {
// Module: crate::rc
// Provides: {"impl_1257"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_array" , since = "1.74.0")] impl < T , const N : usize > From < [T ; N] > for Rc < [T] > { # [doc = " Converts a [`[T; N]`](prim@array) into an `Rc<[T]>`."] # [doc = ""] # [doc = " The conversion moves the array into a newly allocated `Rc`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::rc::Rc;"] # [doc = " let original: [i32; 3] = [1, 2, 3];"] # [doc = " let shared: Rc<[i32]> = Rc::from(original);"] # [doc = " assert_eq!(&[1, 2, 3], &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : [T ; N]) -> Rc < [T] > { Rc :: < [T ; N] > :: from (v) } }
};
}
