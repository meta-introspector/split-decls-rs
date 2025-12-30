// Generated macro for impl_1258 (impl)
macro_rules! Depcrate_rcimpl_1258 {
() => {
// Module: crate::rc
// Provides: {"impl_1258"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_slice" , since = "1.21.0")] impl < T : Clone > From < & [T] > for Rc < [T] > { # [doc = " Allocates a reference-counted slice and fills it by cloning `v`'s items."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::rc::Rc;"] # [doc = " let original: &[i32] = &[1, 2, 3];"] # [doc = " let shared: Rc<[i32]> = Rc::from(original);"] # [doc = " assert_eq!(&[1, 2, 3], &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : & [T]) -> Rc < [T] > { < Self as RcFromSlice < T > > :: from_slice (v) } }
};
}
