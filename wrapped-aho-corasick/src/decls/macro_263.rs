macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
        AhoCorasickKind!();
    };
}

macro_rules! macro_263 {
    () => {
        deps!();
        # [cfg (feature = "std")] testconfig ! (stream , search_standard_stream_nfa_contig_default , AC_STANDARD_NON_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: ContiguousNFA)) ; }) ;
    };
}

macro_263!()