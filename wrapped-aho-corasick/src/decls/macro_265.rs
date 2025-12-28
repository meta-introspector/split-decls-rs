macro_rules! deps {
    () => {
        StartKind!();
        Anchored!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_265 {
    () => {
        deps!();
        testconfig ! (anchored , search_standard_anchored_default , AC_STANDARD_ANCHORED_NON_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) ; }) ;
    };
}

macro_265!()