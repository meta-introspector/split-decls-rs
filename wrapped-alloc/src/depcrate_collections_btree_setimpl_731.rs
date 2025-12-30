// Generated macro for impl_731 (impl)
macro_rules! Depcrate_collections_btree_setimpl_731 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_731"}
// Dependencies: {}
impl < T > BTreeSet < T > { # [doc = " Makes a new, empty `BTreeSet`."] # [doc = ""] # [doc = " Does not allocate anything on its own."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #![allow(unused_mut)]"] # [doc = " use std::collections::BTreeSet;"] # [doc = ""] # [doc = " let mut set: BTreeSet<i32> = BTreeSet::new();"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_const_stable (feature = "const_btree_new" , since = "1.66.0")] # [must_use] pub const fn new () -> BTreeSet < T > { BTreeSet { map : BTreeMap :: new () } } }
};
}
