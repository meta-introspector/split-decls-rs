// Generated macro for tests (module)
macro_rules! Depcrate_dfa_sparsetests {
() => {
// Module: crate::dfa::sparse
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "syntax" , feature = "dfa-build"))] mod tests { use crate :: { dfa :: { dense :: DFA , Automaton } , nfa :: thompson , Input , MatchError , } ; # [test] fn heuristic_unicode_forward () { let dfa = DFA :: builder () . configure (DFA :: config () . unicode_word_boundary (true)) . thompson (thompson :: Config :: new () . reverse (true)) . build (r"\b[0-9]+\b") . unwrap () . to_sparse () . unwrap () ; let input = Input :: new ("β123") . range (2 ..) ; let expected = MatchError :: quit (0xB2 , 1) ; let got = dfa . try_search_fwd (& input) ; assert_eq ! (Err (expected) , got) ; let input = Input :: new ("123β") . range (.. 3) ; let expected = MatchError :: quit (0xCE , 3) ; let got = dfa . try_search_fwd (& input) ; assert_eq ! (Err (expected) , got) ; } # [test] fn heuristic_unicode_reverse () { let dfa = DFA :: builder () . configure (DFA :: config () . unicode_word_boundary (true)) . thompson (thompson :: Config :: new () . reverse (true)) . build (r"\b[0-9]+\b") . unwrap () . to_sparse () . unwrap () ; let input = Input :: new ("β123") . range (2 ..) ; let expected = MatchError :: quit (0xB2 , 1) ; let got = dfa . try_search_rev (& input) ; assert_eq ! (Err (expected) , got) ; let input = Input :: new ("123β") . range (.. 3) ; let expected = MatchError :: quit (0xCE , 3) ; let got = dfa . try_search_rev (& input) ; assert_eq ! (Err (expected) , got) ; } }
};
}
