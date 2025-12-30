// Generated macro for testconfig (macro)
macro_rules! Depcrate_packed_teststestconfig {
() => {
// Module: crate::packed::tests
// Provides: {"testconfig"}
// Dependencies: {}
macro_rules ! testconfig { ($ name : ident , $ collection : expr , $ with : expr) => { # [test] fn $ name () { run_search_tests ($ collection , | test | { let mut config = Config :: new () ; $ with (& mut config) ; let mut builder = config . builder () ; builder . extend (test . patterns . iter () . map (| p | p . as_bytes ())) ; let searcher = match builder . build () { Some (searcher) => searcher , None => { if cfg ! (any (target_arch = "x86_64" , target_arch = "aarch64")) { panic ! ("failed to build packed searcher") } return None ; } } ; Some (searcher . find_iter (& test . haystack) . collect ()) }) ; } } ; }
};
}
