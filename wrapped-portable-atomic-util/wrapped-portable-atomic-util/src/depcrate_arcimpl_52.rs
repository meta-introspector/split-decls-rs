// Generated macro for impl_52 (impl)
macro_rules! Depcrate_arcimpl_52 {
() => {
// Module: crate::arc
// Provides: {"impl_52"}
// Dependencies: {}
impl < T > Default for Weak < T > { # [doc = " Constructs a new `Weak<T>`, without allocating memory."] # [doc = " Calling [`upgrade`] on the return value always"] # [doc = " gives [`None`]."] # [doc = ""] # [doc = " [`upgrade`]: Weak::upgrade"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Weak;"] # [doc = ""] # [doc = " let empty: Weak<i64> = Default::default();"] # [doc = " assert!(empty.upgrade().is_none());"] # [doc = " ```"] fn default () -> Self { Self :: new () } }
};
}
