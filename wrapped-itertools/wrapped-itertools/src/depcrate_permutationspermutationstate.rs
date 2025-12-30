// Generated macro for PermutationState (enum)
macro_rules! Depcrate_permutationsPermutationState {
() => {
// Module: crate::permutations
// Provides: {"PermutationState"}
// Dependencies: {}
# [derive (Clone , Debug)] enum PermutationState { # [doc = " No permutation generated yet."] Start { k : usize } , # [doc = " Values from the iterator are not fully loaded yet so `n` is still unknown."] Buffered { k : usize , min_n : usize } , # [doc = " All values from the iterator are known so `n` is known."] Loaded { indices : Box < [usize] > , cycles : Box < [usize] > , } , # [doc = " No permutation left to generate."] End , }
};
}
