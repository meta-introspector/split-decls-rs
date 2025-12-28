macro_rules! deps {
    () => {
        TryMatchesIter!();
        Match!();
        Input!();
    };
}

macro_rules! MatchesIter {
    () => {
        deps!();
        # [doc = " An iterator over all non-overlapping matches for an infallible search."] # [doc = ""] # [doc = " The iterator yields a [`Match`] value until no more matches could be found."] # [doc = ""] # [doc = " The type parameters are as follows:"] # [doc = ""] # [doc = " * `F` represents the type of a closure that executes the search."] # [doc = ""] # [doc = " The lifetime parameters come from the [`Input`] type:"] # [doc = ""] # [doc = " * `'h` is the lifetime of the underlying haystack."] # [doc = ""] # [doc = " When possible, prefer the iterators defined on the regex engine you're"] # [doc = " using. This tries to abstract over the regex engine and is thus a bit more"] # [doc = " unwieldy to use."] # [doc = ""] # [doc = " This iterator is created by [`Searcher::into_matches_iter`] and"] # [doc = " then calling [`TryMatchesIter::infallible`]."] # [derive (Debug)] pub struct MatchesIter < 'h , F > (TryMatchesIter < 'h , F >) ;
    };
}

MatchesIter!();