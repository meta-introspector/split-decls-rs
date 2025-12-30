// Generated macro for DominatorsIter (struct)
macro_rules! Depcrate_algo_dominatorsDominatorsIter {
() => {
// Module: crate::algo::dominators
// Provides: {"DominatorsIter"}
// Dependencies: {}
# [doc = " Iterator for a node's dominators."] # [derive (Debug , Clone)] pub struct DominatorsIter < 'a , N > where N : 'a + Copy + Eq + Hash , { dominators : & 'a Dominators < N > , node : Option < N > , }
};
}
