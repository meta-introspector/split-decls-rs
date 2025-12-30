// Generated macro for impl_61 (impl)
macro_rules! Depcrate_arcimpl_61 {
() => {
// Module: crate::arc
// Provides: {"impl_61"}
// Dependencies: {}
impl < T : Default > Default for Arc < T > { # [doc = " Creates a new `Arc<T>`, with the `Default` value for `T`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = ""] # [doc = " let x: Arc<i32> = Default::default();"] # [doc = " assert_eq!(*x, 0);"] # [doc = " ```"] fn default () -> Self { Self :: new (T :: default ()) } }
};
}
