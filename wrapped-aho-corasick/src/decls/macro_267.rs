macro_rules! deps {
    () => {
        Anchored!();
        StartKind!();
        AhoCorasickKind!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_267 {
    () => {
        deps!();
        testconfig ! (anchored , search_standard_anchored_nfa_contig_default , AC_STANDARD_ANCHORED_NON_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: ContiguousNFA)) ; }) ;
    };
}

macro_267!();