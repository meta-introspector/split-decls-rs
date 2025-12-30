// Generated macro for Dominators (struct)
macro_rules! Depcrate_algo_dominatorsDominators {
() => {
// Module: crate::algo::dominators
// Provides: {"Dominators"}
// Dependencies: {}
# [doc = " The dominance relation for some graph and root."] # [derive (Debug , Clone)] pub struct Dominators < N > where N : Copy + Eq + Hash , { root : N , dominators : HashMap < N , N > , }
};
}
