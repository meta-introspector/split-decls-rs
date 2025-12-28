macro_rules! deps {
    () => {
        MatchError!();
        Searcher!();
        Input!();
    };
}

macro_rules! TryHalfMatchesIter {
    () => {
        deps!();
        # [doc = " An iterator over all non-overlapping half matches for a fallible search."] # [doc = ""] # [doc = " The iterator yields a `Result<HalfMatch, MatchError>` value until no more"] # [doc = " matches could be found."] # [doc = ""] # [doc = " The type parameters are as follows:"] # [doc = ""] # [doc = " * `F` represents the type of a closure that executes the search."] # [doc = ""] # [doc = " The lifetime parameters come from the [`Input`] type:"] # [doc = ""] # [doc = " * `'h` is the lifetime of the underlying haystack."] # [doc = ""] # [doc = " When possible, prefer the iterators defined on the regex engine you're"] # [doc = " using. This tries to abstract over the regex engine and is thus a bit more"] # [doc = " unwieldy to use."] # [doc = ""] # [doc = " This iterator is created by [`Searcher::into_half_matches_iter`]."] pub struct TryHalfMatchesIter < 'h , F > { it : Searcher < 'h > , finder : F , }
    };
}

TryHalfMatchesIter!()