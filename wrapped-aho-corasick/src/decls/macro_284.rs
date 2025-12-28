macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_284 {
    () => {
        deps!();
        testconfig ! (overlapping , acasei_standard_overlapping_default , & [ASCII_CASE_INSENSITIVE , ASCII_CASE_INSENSITIVE_OVERLAPPING] , Standard , | b : & mut AhoCorasickBuilder | { b . ascii_case_insensitive (true) ; }) ;
    };
}

macro_284!();