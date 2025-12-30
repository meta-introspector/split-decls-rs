// Generated macro for impl_248 (impl)
macro_rules! Depcrate_matcher_support_zipped_iteratorimpl_248 {
() => {
// Module: crate::matcher_support::zipped_iterator
// Provides: {"impl_248"}
// Dependencies: {}
impl < I1 : Iterator , I2 > ZippedIterator < I1 , I2 > { # [doc = " Returns whether a mismatch in the two sizes of the two iterators was"] # [doc = " detected during iteration."] # [doc = ""] # [doc = " This returns `true` if and only if, at some previous call to"] # [doc = " [`Iterator::next`] on this instance, one of the constituent iterators"] # [doc = " had a next element and the other did not."] # [doc = ""] # [doc = " [`Iterator::next`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next"] pub (crate) fn has_size_mismatch (& self) -> bool { self . has_size_mismatch } # [doc = " Returns the number of elements in the left iterator."] # [doc = ""] # [doc = " This iterates through the remainder of the left iterator if necessary in"] # [doc = " order to get the true number of elements. It therefore consumes `self`."] pub (crate) fn left_size (mut self) -> usize { self . consumed_elements + self . left . by_ref () . count () } }
};
}
