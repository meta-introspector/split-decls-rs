macro_rules! deps {
    () => {
        AhoCorasickKind!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_249 {
    () => {
        deps!();
        testconfig ! (overlapping , search_standard_overlapping_nfa_noncontig_default , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) ; }) ;
    };
}

macro_249!()