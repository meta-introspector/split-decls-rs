macro_rules! deps {
    () => {
        BoundedBacktracker!();
        MatchError!();
        Captures!();
        Searcher!();
        Cache!();
    };
}

macro_rules! TryCapturesMatches {
    () => {
        deps!();
        # [doc = " An iterator over all non-overlapping leftmost matches, with their capturing"] # [doc = " groups, for a fallible search."] # [doc = ""] # [doc = " The iterator yields a `Result<Captures, MatchError>` value until no more"] # [doc = " matches could be found."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'r` represents the lifetime of the BoundedBacktracker."] # [doc = " * `'c` represents the lifetime of the BoundedBacktracker's cache."] # [doc = " * `'h` represents the lifetime of the haystack being searched."] # [doc = ""] # [doc = " This iterator can be created with the"] # [doc = " [`BoundedBacktracker::try_captures_iter`] method."] # [derive (Debug)] pub struct TryCapturesMatches < 'r , 'c , 'h > { re : & 'r BoundedBacktracker , cache : & 'c mut Cache , caps : Captures , it : iter :: Searcher < 'h > , }
    };
}

TryCapturesMatches!()