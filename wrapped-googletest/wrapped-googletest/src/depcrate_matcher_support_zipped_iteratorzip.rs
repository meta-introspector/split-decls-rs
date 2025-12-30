// Generated macro for zip (function)
macro_rules! Depcrate_matcher_support_zipped_iteratorzip {
() => {
// Module: crate::matcher_support::zipped_iterator
// Provides: {"zip"}
// Dependencies: {}
# [doc = " Zips up two iterators into a single iterator of pairs."] # [doc = ""] # [doc = " This is identical to [`Iterator::zip`] except that this version allows the"] # [doc = " caller to determine whether the two iterators had mismatching sizes using"] # [doc = " the method [`ZippedIterator::has_size_mismatch`]."] # [doc = ""] # [doc = " [`Iterator::zip`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip"] pub (crate) fn zip < I1 , I2 > (left : I1 , right : I2) -> ZippedIterator < I1 , I2 > { ZippedIterator { left , right , has_size_mismatch : false , consumed_elements : 0 } }
};
}
