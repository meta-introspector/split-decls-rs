macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        StartKind!();
        DFA!();
        AhoCorasickKind!();
    };
}

macro_rules! macro_269 {
    () => {
        deps!();
        testconfig ! (anchored , search_standard_anchored_dfa_start_both , AC_STANDARD_ANCHORED_NON_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Both) . kind (Some (AhoCorasickKind :: DFA)) ; }) ;
    };
}

macro_269!();