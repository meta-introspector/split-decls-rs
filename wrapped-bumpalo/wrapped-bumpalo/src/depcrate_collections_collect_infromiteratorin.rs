// Generated macro for FromIteratorIn (trait)
macro_rules! Depcrate_collections_collect_inFromIteratorIn {
() => {
// Module: crate::collections::collect_in
// Provides: {"FromIteratorIn"}
// Dependencies: {}
# [doc = " A trait for types that support being constructed from an iterator, parameterized by an allocator."] pub trait FromIteratorIn < A > { # [doc = " The allocator type"] type Alloc ; # [doc = " Similar to [`FromIterator::from_iter`][from_iter], but with a given allocator."] # [doc = ""] # [doc = " [from_iter]: https://doc.rust-lang.org/std/iter/trait.FromIterator.html#tymethod.from_iter"] # [doc = ""] # [doc = " ```"] # [doc = " # use bumpalo::collections::{FromIteratorIn, Vec};"] # [doc = " # use bumpalo::Bump;"] # [doc = " #"] # [doc = " let five_fives = std::iter::repeat(5).take(5);"] # [doc = " let bump = Bump::new();"] # [doc = ""] # [doc = " let v = Vec::from_iter_in(five_fives, &bump);"] # [doc = ""] # [doc = " assert_eq!(v, [5, 5, 5, 5, 5]);"] # [doc = " ```"] fn from_iter_in < I > (iter : I , alloc : Self :: Alloc) -> Self where I : IntoIterator < Item = A > ; }
};
}
