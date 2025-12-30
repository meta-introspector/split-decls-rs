// Generated macro for impl_96 (impl)
macro_rules! Depcrate_collections_vecimpl_96 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'bump , T : 'bump + PartialEq > Vec < 'bump , T > { # [doc = " Removes consecutive repeated elements in the vector according to the"] # [doc = " [`PartialEq`] trait implementation."] # [doc = ""] # [doc = " If the vector is sorted, this removes all duplicates."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bumpalo::{Bump, collections::Vec};"] # [doc = ""] # [doc = " let b = Bump::new();"] # [doc = ""] # [doc = " let mut vec = bumpalo::vec![in &b; 1, 2, 2, 3, 2];"] # [doc = ""] # [doc = " vec.dedup();"] # [doc = ""] # [doc = " assert_eq!(vec, [1, 2, 3, 2]);"] # [doc = " ```"] # [inline] pub fn dedup (& mut self) { self . dedup_by (| a , b | a == b) } }
};
}
