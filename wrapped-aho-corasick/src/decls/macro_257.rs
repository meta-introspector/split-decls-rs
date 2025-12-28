macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        AhoCorasickKind!();
        DFA!();
    };
}

macro_rules! macro_257 {
    () => {
        deps!();
        testconfig ! (overlapping , search_standard_overlapping_dfa_no_prefilter , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: DFA)) . prefilter (false) ; }) ;
    };
}

macro_257!()