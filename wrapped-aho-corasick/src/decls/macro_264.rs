macro_rules! deps {
    () => {
        DFA!();
        AhoCorasickKind!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_264 {
    () => {
        deps!();
        # [cfg (feature = "std")] testconfig ! (stream , search_standard_stream_dfa_default , AC_STANDARD_NON_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: DFA)) ; }) ;
    };
}

macro_264!();