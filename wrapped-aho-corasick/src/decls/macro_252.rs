macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        AhoCorasickKind!();
    };
}

macro_rules! macro_252 {
    () => {
        deps!();
        testconfig ! (overlapping , search_standard_overlapping_nfa_contig_no_prefilter , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: ContiguousNFA)) . prefilter (false) ; }) ;
    };
}

macro_252!();