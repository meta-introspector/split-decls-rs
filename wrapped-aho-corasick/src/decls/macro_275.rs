macro_rules! deps {
    () => {
        Anchored!();
        AhoCorasickBuilder!();
        StartKind!();
    };
}

macro_rules! macro_275 {
    () => {
        deps!();
        testconfig ! (anchored , search_leftmost_longest_anchored_default , AC_LEFTMOST_LONGEST_ANCHORED , LeftmostLongest , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) ; }) ;
    };
}

macro_275!();