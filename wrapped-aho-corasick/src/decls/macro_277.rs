macro_rules! deps {
    () => {
        AhoCorasickKind!();
        StartKind!();
        AhoCorasickBuilder!();
        Anchored!();
    };
}

macro_rules! macro_277 {
    () => {
        deps!();
        testconfig ! (anchored , search_leftmost_longest_anchored_nfa_contig_default , AC_LEFTMOST_LONGEST_ANCHORED , LeftmostLongest , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: ContiguousNFA)) ; }) ;
    };
}

macro_277!()