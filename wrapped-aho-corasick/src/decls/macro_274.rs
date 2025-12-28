macro_rules! deps {
    () => {
        AhoCorasickKind!();
        DFA!();
        StartKind!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_274 {
    () => {
        deps!();
        testconfig ! (anchored , search_leftmost_first_anchored_dfa_start_both , AC_LEFTMOST_FIRST_ANCHORED , LeftmostFirst , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Both) . kind (Some (AhoCorasickKind :: DFA)) ; }) ;
    };
}

macro_274!()