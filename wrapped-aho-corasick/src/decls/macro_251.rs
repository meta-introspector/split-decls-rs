macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        AhoCorasickKind!();
    };
}

macro_rules! macro_251 {
    () => {
        deps!();
        testconfig ! (overlapping , search_standard_overlapping_nfa_contig_default , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: ContiguousNFA)) ; }) ;
    };
}

macro_251!();