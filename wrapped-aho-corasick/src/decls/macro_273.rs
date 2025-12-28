macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        DFA!();
        Anchored!();
        StartKind!();
        AhoCorasickKind!();
    };
}

macro_rules! macro_273 {
    () => {
        deps!();
        testconfig ! (anchored , search_leftmost_first_anchored_dfa_default , AC_LEFTMOST_FIRST_ANCHORED , LeftmostFirst , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: DFA)) ; }) ;
    };
}

macro_273!();