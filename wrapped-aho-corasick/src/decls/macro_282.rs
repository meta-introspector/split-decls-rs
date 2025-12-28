macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        AhoCorasickKind!();
    };
}

macro_rules! macro_282 {
    () => {
        deps!();
        testconfig ! (acasei_standard_nfa_contig_default , & [ASCII_CASE_INSENSITIVE] , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: ContiguousNFA)) . prefilter (false) . ascii_case_insensitive (true) ; }) ;
    };
}

macro_282!()