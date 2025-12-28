macro_rules! deps {
    () => {
        StartKind!();
        Anchored!();
        AhoCorasickKind!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_271 {
    () => {
        deps!();
        testconfig ! (anchored , search_leftmost_first_anchored_nfa_noncontig_default , AC_LEFTMOST_FIRST_ANCHORED , LeftmostFirst , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) ; }) ;
    };
}

macro_271!()