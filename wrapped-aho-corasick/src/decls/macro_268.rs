macro_rules! deps {
    () => {
        AhoCorasickKind!();
        AhoCorasickBuilder!();
        StartKind!();
        DFA!();
        Anchored!();
    };
}

macro_rules! macro_268 {
    () => {
        deps!();
        testconfig ! (anchored , search_standard_anchored_dfa_default , AC_STANDARD_ANCHORED_NON_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: DFA)) ; }) ;
    };
}

macro_268!()