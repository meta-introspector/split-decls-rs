macro_rules! deps {
    () => {
        AhoCorasickKind!();
        AhoCorasickBuilder!();
        DFA!();
        StartKind!();
    };
}

macro_rules! macro_258 {
    () => {
        deps!();
        testconfig ! (overlapping , search_standard_overlapping_dfa_start_both_no_prefilter , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: DFA)) . start_kind (StartKind :: Both) . prefilter (false) ; }) ;
    };
}

macro_258!()