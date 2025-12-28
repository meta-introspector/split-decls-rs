macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        AhoCorasickKind!();
    };
}

macro_rules! macro_281 {
    () => {
        deps!();
        testconfig ! (acasei_standard_nfa_noncontig_default , & [ASCII_CASE_INSENSITIVE] , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) . prefilter (false) . ascii_case_insensitive (true) ; }) ;
    };
}

macro_281!()