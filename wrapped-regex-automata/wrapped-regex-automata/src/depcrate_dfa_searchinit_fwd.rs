// Generated macro for init_fwd (function)
macro_rules! Depcrate_dfa_searchinit_fwd {
() => {
// Module: crate::dfa::search
// Provides: {"init_fwd"}
// Dependencies: {}
# [cfg_attr (feature = "perf-inline" , inline (always))] fn init_fwd < A : Automaton + ? Sized > (dfa : & A , input : & Input < '_ > ,) -> Result < StateID , MatchError > { let sid = dfa . start_state_forward (input) ? ; debug_assert ! (! dfa . is_match_state (sid)) ; Ok (sid) }
};
}
