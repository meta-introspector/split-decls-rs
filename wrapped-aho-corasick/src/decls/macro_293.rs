macro_rules! deps {
    () => {
        AhoCorasickKind!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_293 {
    () => {
        deps!();
        testconfig ! (acasei_leftmost_longest_nfa_noncontig_default , & [ASCII_CASE_INSENSITIVE , ASCII_CASE_INSENSITIVE_NON_OVERLAPPING] , LeftmostLongest , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) . ascii_case_insensitive (true) ; }) ;
    };
}

macro_293!();