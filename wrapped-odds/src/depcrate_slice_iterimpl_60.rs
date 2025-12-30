// Generated macro for impl_60 (impl)
macro_rules! Depcrate_slice_iterimpl_60 {
() => {
// Module: crate::slice::iter
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a , T > From < & 'a [T] > for SliceCopyIter < 'a , T > where T : Copy , { fn from (slice : & 'a [T]) -> Self { assert ! (size_of ::< T > () != 0) ; unsafe { let ptr = slice . as_ptr () ; let end = ptr . offset (slice . len () as isize) ; SliceCopyIter :: new (ptr , end) } } }
};
}
