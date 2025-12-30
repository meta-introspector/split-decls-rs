// Generated macro for run_search_tests (function)
macro_rules! Depcrate_testsrun_search_tests {
() => {
// Module: crate::tests
// Provides: {"run_search_tests"}
// Dependencies: {}
fn run_search_tests < F : FnMut (& SearchTest) -> Vec < Match > > (which : TestCollection , mut f : F ,) { let get_match_triples = | matches : Vec < Match > | -> Vec < (usize , usize , usize) > { matches . into_iter () . map (| m | (m . pattern () . as_usize () , m . start () , m . end ())) . collect () } ; for & tests in which { for test in tests { assert_eq ! (test . matches , get_match_triples (f (& test)) . as_slice () , "test: {}, patterns: {:?}, haystack: {:?}" , test . name , test . patterns , test . haystack) ; } } }
};
}
