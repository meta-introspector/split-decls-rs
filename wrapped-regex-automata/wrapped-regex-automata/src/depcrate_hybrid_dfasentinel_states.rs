// Generated macro for SENTINEL_STATES (const)
macro_rules! Depcrate_hybrid_dfaSENTINEL_STATES {
() => {
// Module: crate::hybrid::dfa
// Provides: {"SENTINEL_STATES"}
// Dependencies: {}
# [doc = " The number of \"sentinel\" states that get added to every lazy DFA."] # [doc = ""] # [doc = " These are special states indicating status conditions of a search: unknown,"] # [doc = " dead and quit. These states in particular also use zero NFA states, so"] # [doc = " their memory usage is quite small. This is relevant for computing the"] # [doc = " minimum memory needed for a lazy DFA cache."] const SENTINEL_STATES : usize = 3 ;
};
}
