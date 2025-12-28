macro_rules! deps {
    () => {
        AhoCorasickKind!();
        Anchored!();
        AhoCorasickBuilder!();
        StartKind!();
    };
}

macro_rules! macro_266 {
    () => {
        deps!();
        testconfig ! (anchored , search_standard_anchored_nfa_noncontig_default , AC_STANDARD_ANCHORED_NON_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) ; }) ;
    };
}

macro_266!()