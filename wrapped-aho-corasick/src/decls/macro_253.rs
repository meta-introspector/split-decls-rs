macro_rules! deps {
    () => {
        AhoCorasickKind!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_253 {
    () => {
        deps!();
        testconfig ! (overlapping , search_standard_overlapping_nfa_contig_all_sparse , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: ContiguousNFA)) . dense_depth (0) ; }) ;
    };
}

macro_253!()