// Generated macro for impl_1287 (impl)
macro_rules! Depcrate_rcimpl_1287 {
() => {
// Module: crate::rc
// Provides: {"impl_1287"}
// Dependencies: {}
# [stable (feature = "downgraded_weak" , since = "1.10.0")] impl < T > Default for Weak < T > { # [doc = " Constructs a new `Weak<T>`, without allocating any memory."] # [doc = " Calling [`upgrade`] on the return value always gives [`None`]."] # [doc = ""] # [doc = " [`upgrade`]: Weak::upgrade"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::rc::Weak;"] # [doc = ""] # [doc = " let empty: Weak<i64> = Default::default();"] # [doc = " assert!(empty.upgrade().is_none());"] # [doc = " ```"] fn default () -> Weak < T > { Weak :: new () } }
};
}
