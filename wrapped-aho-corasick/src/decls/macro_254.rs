macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        AhoCorasickKind!();
    };
}

macro_rules! macro_254 {
    () => {
        deps!();
        testconfig ! (overlapping , search_standard_overlapping_nfa_contig_all_dense , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: ContiguousNFA)) . dense_depth (usize :: MAX) ; }) ;
    };
}

macro_254!();