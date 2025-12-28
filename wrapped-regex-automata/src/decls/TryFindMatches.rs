macro_rules! deps {
    () => {
        Cache!();
        Captures!();
        Searcher!();
        MatchError!();
        BoundedBacktracker!();
    };
}

macro_rules! TryFindMatches {
    () => {
        deps!();
        # [doc = " An iterator over all non-overlapping matches for a fallible search."] # [doc = ""] # [doc = " The iterator yields a `Result<Match, MatchError` value until no more"] # [doc = " matches could be found."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'r` represents the lifetime of the BoundedBacktracker."] # [doc = " * `'c` represents the lifetime of the BoundedBacktracker's cache."] # [doc = " * `'h` represents the lifetime of the haystack being searched."] # [doc = ""] # [doc = " This iterator can be created with the [`BoundedBacktracker::try_find_iter`]"] # [doc = " method."] # [derive (Debug)] pub struct TryFindMatches < 'r , 'c , 'h > { re : & 'r BoundedBacktracker , cache : & 'c mut Cache , caps : Captures , it : iter :: Searcher < 'h > , }
    };
}

TryFindMatches!()