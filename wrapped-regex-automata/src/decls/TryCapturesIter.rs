macro_rules! deps {
    () => {
        Searcher!();
        Input!();
        Captures!();
        MatchError!();
    };
}

macro_rules! TryCapturesIter {
    () => {
        deps!();
        # [doc = " An iterator over all non-overlapping captures for a fallible search."] # [doc = ""] # [doc = " The iterator yields a `Result<Captures, MatchError>` value until no more"] # [doc = " matches could be found."] # [doc = ""] # [doc = " The type parameters are as follows:"] # [doc = ""] # [doc = " * `F` represents the type of a closure that executes the search."] # [doc = ""] # [doc = " The lifetime parameters come from the [`Input`] type:"] # [doc = ""] # [doc = " * `'h` is the lifetime of the underlying haystack."] # [doc = ""] # [doc = " When possible, prefer the iterators defined on the regex engine you're"] # [doc = " using. This tries to abstract over the regex engine and is thus a bit more"] # [doc = " unwieldy to use."] # [doc = ""] # [doc = " This iterator is created by [`Searcher::into_captures_iter`]."] # [cfg (feature = "alloc")] pub struct TryCapturesIter < 'h , F > { it : Searcher < 'h > , caps : Captures , finder : F , }
    };
}

TryCapturesIter!()