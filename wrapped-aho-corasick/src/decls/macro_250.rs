macro_rules! deps {
    () => {
        AhoCorasickKind!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_250 {
    () => {
        deps!();
        testconfig ! (overlapping , search_standard_overlapping_nfa_noncontig_no_prefilter , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) . prefilter (false) ; }) ;
    };
}

macro_250!()