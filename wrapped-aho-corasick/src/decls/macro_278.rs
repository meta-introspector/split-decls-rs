macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        StartKind!();
        DFA!();
        Anchored!();
        AhoCorasickKind!();
    };
}

macro_rules! macro_278 {
    () => {
        deps!();
        testconfig ! (anchored , search_leftmost_longest_anchored_dfa_default , AC_LEFTMOST_LONGEST_ANCHORED , LeftmostLongest , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: DFA)) ; }) ;
    };
}

macro_278!()