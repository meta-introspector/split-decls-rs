macro_rules! deps {
    () => {
        SearchTest!();
        TestCollection!();
        Match!();
    };
}

macro_rules! run_search_tests {
    () => {
        deps!();
        fn run_search_tests < F : FnMut (& SearchTest) -> Vec < Match > > (which : TestCollection , mut f : F ,) { let get_match_triples = | matches : Vec < Match > | -> Vec < (usize , usize , usize) > { matches . into_iter () . map (| m | (m . pattern () . as_usize () , m . start () , m . end ())) . collect () } ; for & tests in which { for test in tests { assert_eq ! (test . matches , get_match_triples (f (& test)) . as_slice () , "test: {}, patterns: {:?}, haystack: {:?}" , test . name , test . patterns , test . haystack) ; } } }
    };
}

run_search_tests!()