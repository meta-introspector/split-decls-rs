// Generated macro for CapturesIter (struct)
macro_rules! Depcrate_util_iterCapturesIter {
() => {
// Module: crate::util::iter
// Provides: {"CapturesIter"}
// Dependencies: {}
# [doc = " An iterator over all non-overlapping captures for an infallible search."] # [doc = ""] # [doc = " The iterator yields a [`Captures`] value until no more matches could be"] # [doc = " found."] # [doc = ""] # [doc = " The type parameters are as follows:"] # [doc = ""] # [doc = " * `F` represents the type of a closure that executes the search."] # [doc = ""] # [doc = " The lifetime parameters come from the [`Input`] type:"] # [doc = ""] # [doc = " * `'h` is the lifetime of the underlying haystack."] # [doc = ""] # [doc = " When possible, prefer the iterators defined on the regex engine you're"] # [doc = " using. This tries to abstract over the regex engine and is thus a bit more"] # [doc = " unwieldy to use."] # [doc = ""] # [doc = " This iterator is created by [`Searcher::into_captures_iter`] and then"] # [doc = " calling [`TryCapturesIter::infallible`]."] # [cfg (feature = "alloc")] # [derive (Debug)] pub struct CapturesIter < 'h , F > (TryCapturesIter < 'h , F >) ;
};
}
