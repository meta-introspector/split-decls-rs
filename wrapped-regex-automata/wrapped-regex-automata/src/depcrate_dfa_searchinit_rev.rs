// Generated macro for init_rev (function)
macro_rules! Depcrate_dfa_searchinit_rev {
() => {
// Module: crate::dfa::search
// Provides: {"init_rev"}
// Dependencies: {}
# [cfg_attr (feature = "perf-inline" , inline (always))] fn init_rev < A : Automaton + ? Sized > (dfa : & A , input : & Input < '_ > ,) -> Result < StateID , MatchError > { let sid = dfa . start_state_reverse (input) ? ; debug_assert ! (! dfa . is_match_state (sid)) ; Ok (sid) }
};
}
