macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        DFA!();
        AhoCorasickKind!();
    };
}

macro_rules! macro_295 {
    () => {
        deps!();
        testconfig ! (acasei_leftmost_longest_dfa_default , & [ASCII_CASE_INSENSITIVE , ASCII_CASE_INSENSITIVE_NON_OVERLAPPING] , LeftmostLongest , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: DFA)) . ascii_case_insensitive (true) ; }) ;
    };
}

macro_295!();