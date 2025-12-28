macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        AhoCorasickKind!();
    };
}

macro_rules! macro_285 {
    () => {
        deps!();
        testconfig ! (overlapping , acasei_standard_overlapping_nfa_noncontig_default , & [ASCII_CASE_INSENSITIVE , ASCII_CASE_INSENSITIVE_OVERLAPPING] , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) . ascii_case_insensitive (true) ; }) ;
    };
}

macro_285!()