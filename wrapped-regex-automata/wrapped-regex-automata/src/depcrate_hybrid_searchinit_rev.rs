// Generated macro for init_rev (function)
macro_rules! Depcrate_hybrid_searchinit_rev {
() => {
// Module: crate::hybrid::search
// Provides: {"init_rev"}
// Dependencies: {}
# [cfg_attr (feature = "perf-inline" , inline (always))] fn init_rev (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > ,) -> Result < LazyStateID , MatchError > { let sid = dfa . start_state_reverse (cache , input) ? ; debug_assert ! (! sid . is_match ()) ; Ok (sid) }
};
}
