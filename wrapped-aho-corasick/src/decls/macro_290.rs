macro_rules! deps {
    () => {
        AhoCorasickKind!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_290 {
    () => {
        deps!();
        testconfig ! (acasei_leftmost_first_nfa_contig_default , & [ASCII_CASE_INSENSITIVE , ASCII_CASE_INSENSITIVE_NON_OVERLAPPING] , LeftmostFirst , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: ContiguousNFA)) . ascii_case_insensitive (true) ; }) ;
    };
}

macro_290!();