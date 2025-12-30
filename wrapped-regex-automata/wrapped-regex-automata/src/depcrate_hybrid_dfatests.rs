// Generated macro for tests (module)
macro_rules! Depcrate_hybrid_dfatests {
() => {
// Module: crate::hybrid::dfa
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "syntax"))] mod tests { use super :: * ; # [test] fn heuristic_unicode_reverse () { let dfa = DFA :: builder () . configure (DFA :: config () . unicode_word_boundary (true)) . thompson (thompson :: Config :: new () . reverse (true)) . build (r"\b[0-9]+\b") . unwrap () ; let mut cache = dfa . create_cache () ; let input = Input :: new ("β123") . range (2 ..) ; let expected = MatchError :: quit (0xB2 , 1) ; let got = dfa . try_search_rev (& mut cache , & input) ; assert_eq ! (Err (expected) , got) ; let input = Input :: new ("123β") . range (.. 3) ; let expected = MatchError :: quit (0xCE , 3) ; let got = dfa . try_search_rev (& mut cache , & input) ; assert_eq ! (Err (expected) , got) ; } }
};
}
