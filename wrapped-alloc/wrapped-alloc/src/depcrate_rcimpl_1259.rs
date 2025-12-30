// Generated macro for impl_1259 (impl)
macro_rules! Depcrate_rcimpl_1259 {
() => {
// Module: crate::rc
// Provides: {"impl_1259"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_mut_slice" , since = "1.84.0")] impl < T : Clone > From < & mut [T] > for Rc < [T] > { # [doc = " Allocates a reference-counted slice and fills it by cloning `v`'s items."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::rc::Rc;"] # [doc = " let mut original = [1, 2, 3];"] # [doc = " let original: &mut [i32] = &mut original;"] # [doc = " let shared: Rc<[i32]> = Rc::from(original);"] # [doc = " assert_eq!(&[1, 2, 3], &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : & mut [T]) -> Rc < [T] > { Rc :: from (& * v) } }
};
}
