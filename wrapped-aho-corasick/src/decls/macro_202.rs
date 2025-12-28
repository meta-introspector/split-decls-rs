macro_rules! deps {
    () => {
        MatchKind!();
        Config!();
    };
}

macro_rules! macro_202 {
    () => {
        deps!();
        testconfig ! (search_rabinkarp_leftmost_longest , PACKED_LEFTMOST_LONGEST , | c : & mut Config | { c . only_rabin_karp (true) . match_kind (MatchKind :: LeftmostLongest) ; }) ;
    };
}

macro_202!()