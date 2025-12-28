macro_rules! deps {
    () => {
        AhoCorasickKind!();
        StartKind!();
        AhoCorasickBuilder!();
        DFA!();
    };
}

macro_rules! macro_256 {
    () => {
        deps!();
        testconfig ! (overlapping , search_standard_overlapping_dfa_start_both , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: DFA)) . start_kind (StartKind :: Both) ; }) ;
    };
}

macro_256!();