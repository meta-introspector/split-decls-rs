// Generated macro for impl_395 (impl)
macro_rules! Depcrate_hash_setimpl_395 {
() => {
// Module: crate::hash::set
// Provides: {"impl_395"}
// Dependencies: {}
impl < A > HashSet < A , RandomState > where A : Hash + Eq + Clone , { # [doc = " Construct a set with a single value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate im;"] # [doc = " # use im::hashset::HashSet;"] # [doc = " # use std::sync::Arc;"] # [doc = " let set = HashSet::unit(123);"] # [doc = " assert!(set.contains(&123));"] # [doc = " ```"] # [inline] # [must_use] pub fn unit (a : A) -> Self { HashSet :: new () . update (a) } }
};
}
