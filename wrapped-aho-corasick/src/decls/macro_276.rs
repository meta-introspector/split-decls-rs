macro_rules! deps {
    () => {
        Anchored!();
        StartKind!();
        AhoCorasickKind!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_276 {
    () => {
        deps!();
        testconfig ! (anchored , search_leftmost_longest_anchored_nfa_noncontig_default , AC_LEFTMOST_LONGEST_ANCHORED , LeftmostLongest , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) ; }) ;
    };
}

macro_276!()