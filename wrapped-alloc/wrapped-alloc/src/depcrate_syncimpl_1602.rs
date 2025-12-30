// Generated macro for impl_1602 (impl)
macro_rules! Depcrate_syncimpl_1602 {
() => {
// Module: crate::sync
// Provides: {"impl_1602"}
// Dependencies: {}
# [stable (feature = "downgraded_weak" , since = "1.10.0")] impl < T > Default for Weak < T > { # [doc = " Constructs a new `Weak<T>`, without allocating memory."] # [doc = " Calling [`upgrade`] on the return value always"] # [doc = " gives [`None`]."] # [doc = ""] # [doc = " [`upgrade`]: Weak::upgrade"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Weak;"] # [doc = ""] # [doc = " let empty: Weak<i64> = Default::default();"] # [doc = " assert!(empty.upgrade().is_none());"] # [doc = " ```"] fn default () -> Weak < T > { Weak :: new () } }
};
}
