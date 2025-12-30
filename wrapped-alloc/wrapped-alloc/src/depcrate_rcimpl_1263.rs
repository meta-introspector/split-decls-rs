// Generated macro for impl_1263 (impl)
macro_rules! Depcrate_rcimpl_1263 {
() => {
// Module: crate::rc
// Provides: {"impl_1263"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_slice" , since = "1.21.0")] impl < T : ? Sized , A : Allocator > From < Box < T , A > > for Rc < T , A > { # [doc = " Move a boxed object to a new, reference counted, allocation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::rc::Rc;"] # [doc = " let original: Box<i32> = Box::new(1);"] # [doc = " let shared: Rc<i32> = Rc::from(original);"] # [doc = " assert_eq!(1, *shared);"] # [doc = " ```"] # [inline] fn from (v : Box < T , A >) -> Rc < T , A > { Rc :: from_box_in (v) } }
};
}
