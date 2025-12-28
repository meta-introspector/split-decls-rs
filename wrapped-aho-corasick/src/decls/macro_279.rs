macro_rules! deps {
    () => {
        StartKind!();
        AhoCorasickKind!();
        DFA!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_279 {
    () => {
        deps!();
        testconfig ! (anchored , search_leftmost_longest_anchored_dfa_start_both , AC_LEFTMOST_LONGEST_ANCHORED , LeftmostLongest , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Both) . kind (Some (AhoCorasickKind :: DFA)) ; }) ;
    };
}

macro_279!()