// Generated macro for impl_28 (impl)
macro_rules! Depcrate_boxedimpl_28 {
() => {
// Module: crate::boxed
// Provides: {"impl_28"}
// Dependencies: {}
# [cfg (feature = "collections")] impl < 'a , A > Box < 'a , [A] > { # [doc = " Creates a value from an iterator."] # [doc = " This method is an adapted version of [`FromIterator::from_iter`][from_iter]."] # [doc = " It cannot be made as that trait implementation given different signature."] # [doc = ""] # [doc = " [from_iter]: https://doc.rust-lang.org/std/iter/trait.FromIterator.html#tymethod.from_iter"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = " ```"] # [doc = " use bumpalo::{Bump, boxed::Box, vec};"] # [doc = ""] # [doc = " let b = Bump::new();"] # [doc = ""] # [doc = " let five_fives = std::iter::repeat(5).take(5);"] # [doc = " let slice = Box::from_iter_in(five_fives, &b);"] # [doc = " assert_eq!(vec![in &b; 5, 5, 5, 5, 5], &*slice);"] # [doc = " ```"] pub fn from_iter_in < T : IntoIterator < Item = A > > (iter : T , a : & 'a Bump) -> Self { use crate :: collections :: Vec ; let mut vec = Vec :: new_in (a) ; vec . extend (iter) ; vec . into_boxed_slice () } }
};
}
