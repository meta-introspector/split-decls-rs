macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        AhoCorasickKind!();
    };
}

macro_rules! macro_289 {
    () => {
        deps!();
        testconfig ! (acasei_leftmost_first_nfa_noncontig_default , & [ASCII_CASE_INSENSITIVE , ASCII_CASE_INSENSITIVE_NON_OVERLAPPING] , LeftmostFirst , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) . ascii_case_insensitive (true) ; }) ;
    };
}

macro_289!()