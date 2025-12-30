// Generated macro for impl_1239 (impl)
macro_rules! Depcrate_rcimpl_1239 {
() => {
// Module: crate::rc
// Provides: {"impl_1239"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : Default > Default for Rc < T > { # [doc = " Creates a new `Rc<T>`, with the `Default` value for `T`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::rc::Rc;"] # [doc = ""] # [doc = " let x: Rc<i32> = Default::default();"] # [doc = " assert_eq!(*x, 0);"] # [doc = " ```"] # [inline] fn default () -> Self { unsafe { Self :: from_inner (Box :: leak (Box :: write (Box :: new_uninit () , RcInner { strong : Cell :: new (1) , weak : Cell :: new (1) , value : T :: default () } ,)) . into () ,) } } }
};
}
