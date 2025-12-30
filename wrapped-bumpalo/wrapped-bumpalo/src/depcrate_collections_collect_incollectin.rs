// Generated macro for CollectIn (trait)
macro_rules! Depcrate_collections_collect_inCollectIn {
() => {
// Module: crate::collections::collect_in
// Provides: {"CollectIn"}
// Dependencies: {}
# [doc = " Extension trait for iterators, in order to allow allocator-parameterized collections to be constructed more easily."] pub trait CollectIn : Iterator + Sized { # [doc = " Collect all items from an iterator, into a collection parameterized by an allocator."] # [doc = " Similar to [`Iterator::collect`][collect]."] # [doc = ""] # [doc = " [collect]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect"] # [doc = ""] # [doc = " ```"] # [doc = " # use bumpalo::collections::{FromIteratorIn, CollectIn, Vec, String};"] # [doc = " # use bumpalo::Bump;"] # [doc = " #"] # [doc = " let bump = Bump::new();"] # [doc = ""] # [doc = " let str = \"hello, world!\".to_owned();"] # [doc = " let bump_str: String = str.chars().collect_in(&bump);"] # [doc = " assert_eq!(&bump_str, &str);"] # [doc = ""] # [doc = " let nums: Vec<i32> = (0..=3).collect_in::<Vec<_>>(&bump);"] # [doc = " assert_eq!(&nums, &[0,1,2,3]);"] # [doc = " ```"] fn collect_in < C : FromIteratorIn < Self :: Item > > (self , alloc : C :: Alloc) -> C { C :: from_iter_in (self , alloc) } }
};
}
