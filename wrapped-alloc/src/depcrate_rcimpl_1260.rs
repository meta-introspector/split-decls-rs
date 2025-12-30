// Generated macro for impl_1260 (impl)
macro_rules! Depcrate_rcimpl_1260 {
() => {
// Module: crate::rc
// Provides: {"impl_1260"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_slice" , since = "1.21.0")] impl From < & str > for Rc < str > { # [doc = " Allocates a reference-counted string slice and copies `v` into it."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::rc::Rc;"] # [doc = " let shared: Rc<str> = Rc::from(\"statue\");"] # [doc = " assert_eq!(\"statue\", &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : & str) -> Rc < str > { let rc = Rc :: < [u8] > :: from (v . as_bytes ()) ; unsafe { Rc :: from_raw (Rc :: into_raw (rc) as * const str) } } }
};
}
