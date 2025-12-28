macro_rules! deps {
    () => {
        TestCollection!();
        SearchTest!();
        Match!();
    };
}

macro_rules! run_stream_search_tests {
    () => {
        deps!();
        # [cfg (feature = "std")] fn run_stream_search_tests < F : FnMut (& SearchTest) -> Vec < Match > > (which : TestCollection , mut f : F ,) { let get_match_triples = | matches : Vec < Match > | -> Vec < (usize , usize , usize) > { matches . into_iter () . map (| m | (m . pattern () . as_usize () , m . start () , m . end ())) . collect () } ; for & tests in which { for test in tests { if test . patterns . iter () . any (| p | p . is_empty ()) { continue ; } assert_eq ! (test . matches , get_match_triples (f (& test)) . as_slice () , "test: {}, patterns: {:?}, haystack: {:?}" , test . name , test . patterns , test . haystack) ; } } }
    };
}

run_stream_search_tests!()