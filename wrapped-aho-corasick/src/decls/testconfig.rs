macro_rules! deps {
    () => {
        Input!();
        MatchKind!();
        AhoCorasick!();
        Anchored!();
    };
}

macro_rules! testconfig {
    () => {
        deps!();
        macro_rules ! testconfig { (anchored , $ name : ident , $ collection : expr , $ kind : ident , $ with : expr) => { # [test] fn $ name () { run_search_tests ($ collection , | test | { let mut builder = AhoCorasick :: builder () ; $ with (& mut builder) ; let input = Input :: new (test . haystack) . anchored (Anchored :: Yes) ; builder . match_kind (MatchKind ::$ kind) . build (test . patterns) . unwrap () . try_find_iter (input) . unwrap () . collect () }) ; } } ; (overlapping , $ name : ident , $ collection : expr , $ kind : ident , $ with : expr) => { # [test] fn $ name () { run_search_tests ($ collection , | test | { let mut builder = AhoCorasick :: builder () ; $ with (& mut builder) ; builder . match_kind (MatchKind ::$ kind) . build (test . patterns) . unwrap () . find_overlapping_iter (test . haystack) . collect () }) ; } } ; (stream , $ name : ident , $ collection : expr , $ kind : ident , $ with : expr) => { # [test] fn $ name () { run_stream_search_tests ($ collection , | test | { let buf = std :: io :: BufReader :: with_capacity (1 , test . haystack . as_bytes () ,) ; let mut builder = AhoCorasick :: builder () ; $ with (& mut builder) ; builder . match_kind (MatchKind ::$ kind) . build (test . patterns) . unwrap () . stream_find_iter (buf) . map (| result | result . unwrap ()) . collect () }) ; } } ; ($ name : ident , $ collection : expr , $ kind : ident , $ with : expr) => { # [test] fn $ name () { run_search_tests ($ collection , | test | { let mut builder = AhoCorasick :: builder () ; $ with (& mut builder) ; builder . match_kind (MatchKind ::$ kind) . build (test . patterns) . unwrap () . find_iter (test . haystack) . collect () }) ; } } ; }
    };
}

testconfig!();