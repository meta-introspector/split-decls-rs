// Generated macro for permutations (function)
macro_rules! Depcrate_permutationspermutations {
() => {
// Module: crate::permutations
// Provides: {"permutations"}
// Dependencies: {}
pub fn permutations < I : Iterator > (iter : I , k : usize) -> Permutations < I > { Permutations { vals : LazyBuffer :: new (iter) , state : PermutationState :: Start { k } , } }
};
}
