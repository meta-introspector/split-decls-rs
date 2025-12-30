// Generated macro for prefilter_restart (function)
macro_rules! Depcrate_dfa_searchprefilter_restart {
() => {
// Module: crate::dfa::search
// Provides: {"prefilter_restart"}
// Dependencies: {}
# [doc = " Re-compute the starting state that a DFA should be in after finding a"] # [doc = " prefilter candidate match at the position `at`."] # [doc = ""] # [doc = " The function with the same name has a bit more docs in hybrid/search.rs."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn prefilter_restart < A : Automaton + ? Sized > (dfa : & A , input : & Input < '_ > , at : usize ,) -> Result < StateID , MatchError > { let mut input = input . clone () ; input . set_start (at) ; init_fwd (dfa , & input) }
};
}
