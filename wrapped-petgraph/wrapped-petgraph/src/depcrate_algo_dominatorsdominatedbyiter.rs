// Generated macro for DominatedByIter (struct)
macro_rules! Depcrate_algo_dominatorsDominatedByIter {
() => {
// Module: crate::algo::dominators
// Provides: {"DominatedByIter"}
// Dependencies: {}
# [doc = " Iterator for nodes dominated by a given node."] # [derive (Debug , Clone)] pub struct DominatedByIter < 'a , N > where N : 'a + Copy + Eq + Hash , { iter : Iter < 'a , N , N > , node : N , }
};
}
