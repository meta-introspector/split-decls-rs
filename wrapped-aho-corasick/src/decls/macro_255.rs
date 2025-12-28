macro_rules! deps {
    () => {
        DFA!();
        AhoCorasickKind!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_255 {
    () => {
        deps!();
        testconfig ! (overlapping , search_standard_overlapping_dfa_default , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: DFA)) ; }) ;
    };
}

macro_255!();