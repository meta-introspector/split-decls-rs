macro_rules! deps {
    () => {
        AhoCorasickBuilder!();
    };
}

macro_rules! macro_280 {
    () => {
        deps!();
        testconfig ! (acasei_standard_default , & [ASCII_CASE_INSENSITIVE] , Standard , | b : & mut AhoCorasickBuilder | { b . prefilter (false) . ascii_case_insensitive (true) ; }) ;
    };
}

macro_280!()