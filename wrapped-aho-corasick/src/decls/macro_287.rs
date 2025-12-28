macro_rules! deps {
    () => {
        AhoCorasickKind!();
        AhoCorasickBuilder!();
        DFA!();
    };
}

macro_rules! macro_287 {
    () => {
        deps!();
        testconfig ! (overlapping , acasei_standard_overlapping_dfa_default , & [ASCII_CASE_INSENSITIVE , ASCII_CASE_INSENSITIVE_OVERLAPPING] , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: DFA)) . ascii_case_insensitive (true) ; }) ;
    };
}

macro_287!()