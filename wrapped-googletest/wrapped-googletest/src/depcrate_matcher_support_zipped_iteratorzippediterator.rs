// Generated macro for ZippedIterator (struct)
macro_rules! Depcrate_matcher_support_zipped_iteratorZippedIterator {
() => {
// Module: crate::matcher_support::zipped_iterator
// Provides: {"ZippedIterator"}
// Dependencies: {}
# [doc = " An iterator over pairs of the elements of two constituent iterators, which"] # [doc = " keeps track of whether the two iterators have the same size."] # [doc = ""] # [doc = " This is identical to [`Zip`] except that it allows the caller to determine"] # [doc = " whether the two iterators had mismatching sizes using the method"] # [doc = " [`ZippedIterator::has_size_mismatch`]."] # [doc = ""] # [doc = " [`Zip`]: https://doc.rust-lang.org/std/iter/struct.Zip.html"] pub (crate) struct ZippedIterator < I1 , I2 > { left : I1 , right : I2 , has_size_mismatch : bool , consumed_elements : usize , }
};
}
