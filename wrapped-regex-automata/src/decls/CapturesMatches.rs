macro_rules! deps {
    () => {
        Captures!();
        PikeVM!();
        Cache!();
        Searcher!();
    };
}

macro_rules! CapturesMatches {
    () => {
        deps!();
        # [doc = " An iterator over all non-overlapping leftmost matches, with their capturing"] # [doc = " groups, for a particular search."] # [doc = ""] # [doc = " The iterator yields a [`Captures`] value until no more matches could be"] # [doc = " found."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'r` represents the lifetime of the PikeVM."] # [doc = " * `'c` represents the lifetime of the PikeVM's cache."] # [doc = " * `'h` represents the lifetime of the haystack being searched."] # [doc = ""] # [doc = " This iterator can be created with the [`PikeVM::captures_iter`] method."] # [derive (Debug)] pub struct CapturesMatches < 'r , 'c , 'h > { re : & 'r PikeVM , cache : & 'c mut Cache , caps : Captures , it : iter :: Searcher < 'h > , }
    };
}

CapturesMatches!();