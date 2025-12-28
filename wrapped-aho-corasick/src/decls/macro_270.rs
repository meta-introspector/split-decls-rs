macro_rules! deps {
    () => {
        Anchored!();
        AhoCorasickBuilder!();
        StartKind!();
    };
}

macro_rules! macro_270 {
    () => {
        deps!();
        testconfig ! (anchored , search_leftmost_first_anchored_default , AC_LEFTMOST_FIRST_ANCHORED , LeftmostFirst , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) ; }) ;
    };
}

macro_270!()