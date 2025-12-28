macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        AhoCorasickKind!();
    };
}

macro_rules! macro_294 {
    () => {
        deps!();
        testconfig ! (acasei_leftmost_longest_nfa_contig_default , & [ASCII_CASE_INSENSITIVE , ASCII_CASE_INSENSITIVE_NON_OVERLAPPING] , LeftmostLongest , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: ContiguousNFA)) . ascii_case_insensitive (true) ; }) ;
    };
}

macro_294!();