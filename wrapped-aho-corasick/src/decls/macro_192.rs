macro_rules! deps {
    () => {
        Config!();
        MatchKind!();
    };
}

macro_rules! macro_192 {
    () => {
        deps!();
        testconfig ! (search_default_leftmost_longest , PACKED_LEFTMOST_LONGEST , | c : & mut Config | { c . match_kind (MatchKind :: LeftmostLongest) ; }) ;
    };
}

macro_192!();