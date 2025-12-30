// Generated macro for STATE_MAX (const)
macro_rules! Depcrate_dfaSTATE_MAX {
() => {
// Module: crate::dfa
// Provides: {"STATE_MAX"}
// Dependencies: {}
# [doc = " The maximum state pointer. This is useful to mask out the \"valid\" state"] # [doc = " pointer from a state with the \"start\" or \"match\" bits set."] # [doc = ""] # [doc = " It doesn't make sense to use this with unknown, dead or quit state"] # [doc = " pointers, since those pointers are sentinels and never have their lower"] # [doc = " bits set to anything meaningful."] const STATE_MAX : StatePtr = STATE_MATCH - 1 ;
};
}
