macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_292 {
    () => {
        deps!();
        testconfig ! (acasei_leftmost_longest_default , & [ASCII_CASE_INSENSITIVE , ASCII_CASE_INSENSITIVE_NON_OVERLAPPING] , LeftmostLongest , | b : & mut AhoCorasickBuilder | { b . ascii_case_insensitive (true) ; }) ;
    };
}

macro_292!()