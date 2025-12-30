// Generated macro for multipeek (function)
macro_rules! Depcrate_multipeek_implmultipeek {
() => {
// Module: crate::multipeek_impl
// Provides: {"multipeek"}
// Dependencies: {}
# [doc = " An iterator adaptor that allows the user to peek at multiple `.next()`"] # [doc = " values without advancing the base iterator."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::multipeek`]."] pub fn multipeek < I > (iterable : I) -> MultiPeek < I :: IntoIter > where I : IntoIterator , { MultiPeek { iter : iterable . into_iter () . fuse () , buf : VecDeque :: new () , index : 0 , } }
};
}
