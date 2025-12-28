macro_rules! deps {
    () => {
        Searcher!();
        Match!();
        PikeVM!();
        Captures!();
        Cache!();
    };
}

macro_rules! FindMatches {
    () => {
        deps!();
        # [doc = " An iterator over all non-overlapping matches for a particular search."] # [doc = ""] # [doc = " The iterator yields a [`Match`] value until no more matches could be found."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'r` represents the lifetime of the PikeVM."] # [doc = " * `'c` represents the lifetime of the PikeVM's cache."] # [doc = " * `'h` represents the lifetime of the haystack being searched."] # [doc = ""] # [doc = " This iterator can be created with the [`PikeVM::find_iter`] method."] # [derive (Debug)] pub struct FindMatches < 'r , 'c , 'h > { re : & 'r PikeVM , cache : & 'c mut Cache , caps : Captures , it : iter :: Searcher < 'h > , }
    };
}

FindMatches!()