macro_rules! deps {
    () => {
        AhoCorasickKind!();
        DFA!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_283 {
    () => {
        deps!();
        testconfig ! (acasei_standard_dfa_default , & [ASCII_CASE_INSENSITIVE , ASCII_CASE_INSENSITIVE_NON_OVERLAPPING] , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: DFA)) . ascii_case_insensitive (true) ; }) ;
    };
}

macro_283!()