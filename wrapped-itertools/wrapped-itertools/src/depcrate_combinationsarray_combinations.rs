// Generated macro for array_combinations (function)
macro_rules! Depcrate_combinationsarray_combinations {
() => {
// Module: crate::combinations
// Provides: {"array_combinations"}
// Dependencies: {}
# [doc = " Create a new `ArrayCombinations` from a cloneable iterator."] pub fn array_combinations < I : Iterator , const K : usize > (iter : I) -> ArrayCombinations < I , K > where I :: Item : Clone , { ArrayCombinations :: new (iter , array :: from_fn (| i | i)) }
};
}
