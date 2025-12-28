macro_rules! deps {
    () => {
        DFA!();
        AhoCorasickKind!();
        AhoCorasickBuilder!();
        StartKind!();
    };
}

macro_rules! macro_260 {
    () => {
        deps!();
        testconfig ! (overlapping , search_standard_overlapping_dfa_start_both_no_byte_class , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: DFA)) . start_kind (StartKind :: Both) . byte_classes (false) ; }) ;
    };
}

macro_260!();