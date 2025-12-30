// Generated macro for impl_1262 (impl)
macro_rules! Depcrate_rcimpl_1262 {
() => {
// Module: crate::rc
// Provides: {"impl_1262"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_slice" , since = "1.21.0")] impl From < String > for Rc < str > { # [doc = " Allocates a reference-counted string slice and copies `v` into it."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::rc::Rc;"] # [doc = " let original: String = \"statue\".to_owned();"] # [doc = " let shared: Rc<str> = Rc::from(original);"] # [doc = " assert_eq!(\"statue\", &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : String) -> Rc < str > { Rc :: from (& v [..]) } }
};
}
