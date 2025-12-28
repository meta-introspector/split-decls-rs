macro_rules! deps {
    () => {
        AhoCorasickKind!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_262 {
    () => {
        deps!();
        # [cfg (feature = "std")] testconfig ! (stream , search_standard_stream_nfa_noncontig_default , AC_STANDARD_NON_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) ; }) ;
    };
}

macro_262!();