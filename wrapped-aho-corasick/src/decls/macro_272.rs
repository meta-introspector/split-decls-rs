macro_rules! deps {
    () => {
        StartKind!();
        AhoCorasickBuilder!();
        AhoCorasickKind!();
        Anchored!();
    };
}

macro_rules! macro_272 {
    () => {
        deps!();
        testconfig ! (anchored , search_leftmost_first_anchored_nfa_contig_default , AC_LEFTMOST_FIRST_ANCHORED , LeftmostFirst , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: ContiguousNFA)) ; }) ;
    };
}

macro_272!()