// Generated macro for init_fwd (function)
macro_rules! Depcrate_hybrid_searchinit_fwd {
() => {
// Module: crate::hybrid::search
// Provides: {"init_fwd"}
// Dependencies: {}
# [cfg_attr (feature = "perf-inline" , inline (always))] fn init_fwd (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > ,) -> Result < LazyStateID , MatchError > { let sid = dfa . start_state_forward (cache , input) ? ; debug_assert ! (! sid . is_match ()) ; Ok (sid) }
};
}
