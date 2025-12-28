macro_rules! deps {
    () => {
        AhoCorasickKind!();
        DFA!();
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_259 {
    () => {
        deps!();
        testconfig ! (overlapping , search_standard_overlapping_dfa_no_byte_class , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: DFA)) . byte_classes (false) ; }) ;
    };
}

macro_259!()