macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_288 {
    () => {
        deps!();
        testconfig ! (acasei_leftmost_first_default , & [ASCII_CASE_INSENSITIVE , ASCII_CASE_INSENSITIVE_NON_OVERLAPPING] , LeftmostFirst , | b : & mut AhoCorasickBuilder | { b . ascii_case_insensitive (true) ; }) ;
    };
}

macro_288!();