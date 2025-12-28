macro_rules! deps {
    () => {
        AhoCorasickKind!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_286 {
    () => {
        deps!();
        testconfig ! (overlapping , acasei_standard_overlapping_nfa_contig_default , & [ASCII_CASE_INSENSITIVE , ASCII_CASE_INSENSITIVE_OVERLAPPING] , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: ContiguousNFA)) . ascii_case_insensitive (true) ; }) ;
    };
}

macro_286!();