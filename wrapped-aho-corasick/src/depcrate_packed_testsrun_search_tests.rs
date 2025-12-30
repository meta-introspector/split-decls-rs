// Generated macro for run_search_tests (function)
macro_rules! Depcrate_packed_testsrun_search_tests {
() => {
// Module: crate::packed::tests
// Provides: {"run_search_tests"}
// Dependencies: {}
fn run_search_tests < F : FnMut (& SearchTestOwned) -> Option < Vec < Match > > > (which : TestCollection , mut f : F ,) { let get_match_triples = | matches : Vec < Match > | -> Vec < (usize , usize , usize) > { matches . into_iter () . map (| m | (m . pattern () . as_usize () , m . start () , m . end ())) . collect () } ; for & tests in which { for spec in tests { for test in spec . variations () { let results = match f (& test) { None => continue , Some (results) => results , } ; assert_eq ! (test . matches , get_match_triples (results) . as_slice () , "test: {}, patterns: {:?}, haystack(len={:?}): {:?}, \
                     offset: {:?}" , test . name , test . patterns , test . haystack . len () , test . haystack , test . offset ,) ; } } } }
};
}
