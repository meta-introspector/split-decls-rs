// Generated macro for impl_1614 (impl)
macro_rules! Depcrate_syncimpl_1614 {
() => {
// Module: crate::sync
// Provides: {"impl_1614"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : Default > Default for Arc < T > { # [doc = " Creates a new `Arc<T>`, with the `Default` value for `T`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " let x: Arc<i32> = Default::default();"] # [doc = " assert_eq!(*x, 0);"] # [doc = " ```"] fn default () -> Arc < T > { unsafe { Self :: from_inner (Box :: leak (Box :: write (Box :: new_uninit () , ArcInner { strong : atomic :: AtomicUsize :: new (1) , weak : atomic :: AtomicUsize :: new (1) , data : T :: default () , } ,)) . into () ,) } } }
};
}
