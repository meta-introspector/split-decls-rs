macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        AhoCorasickKind!();
        DFA!();
    };
}

macro_rules! macro_291 {
    () => {
        deps!();
        testconfig ! (acasei_leftmost_first_dfa_default , & [ASCII_CASE_INSENSITIVE , ASCII_CASE_INSENSITIVE_NON_OVERLAPPING] , LeftmostFirst , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: DFA)) . ascii_case_insensitive (true) ; }) ;
    };
}

macro_291!()