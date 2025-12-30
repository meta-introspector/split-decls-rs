// Generated macro for array_combinations_with_replacement (function)
macro_rules! Depcrate_combinations_with_replacementarray_combinations_with_replacement {
() => {
// Module: crate::combinations_with_replacement
// Provides: {"array_combinations_with_replacement"}
// Dependencies: {}
# [doc = " Create a new `ArrayCombinationsWithReplacement`` from a cloneable iterator."] pub fn array_combinations_with_replacement < I : Iterator , const K : usize > (iter : I ,) -> ArrayCombinationsWithReplacement < I , K > where I :: Item : Clone , { ArrayCombinationsWithReplacement :: new (iter , [0 ; K]) }
};
}
