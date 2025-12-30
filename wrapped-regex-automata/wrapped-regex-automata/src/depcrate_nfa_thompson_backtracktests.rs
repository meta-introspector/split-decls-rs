// Generated macro for tests (module)
macro_rules! Depcrate_nfa_thompson_backtracktests {
() => {
// Module: crate::nfa::thompson::backtrack
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [cfg (feature = "syntax")] # [test] fn max_haystack_len_overflow () { let re = BoundedBacktracker :: builder () . configure (BoundedBacktracker :: config () . visited_capacity (10)) . build (r"[0-9A-Za-z]{100}") . unwrap () ; assert_eq ! (0 , re . max_haystack_len ()) ; } }
};
}
