// Generated macro for impl_1261 (impl)
macro_rules! Depcrate_rcimpl_1261 {
() => {
// Module: crate::rc
// Provides: {"impl_1261"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_mut_slice" , since = "1.84.0")] impl From < & mut str > for Rc < str > { # [doc = " Allocates a reference-counted string slice and copies `v` into it."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::rc::Rc;"] # [doc = " let mut original = String::from(\"statue\");"] # [doc = " let original: &mut str = &mut original;"] # [doc = " let shared: Rc<str> = Rc::from(original);"] # [doc = " assert_eq!(\"statue\", &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : & mut str) -> Rc < str > { Rc :: from (& * v) } }
};
}
