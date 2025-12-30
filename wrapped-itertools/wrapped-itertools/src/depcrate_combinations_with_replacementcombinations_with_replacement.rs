// Generated macro for combinations_with_replacement (function)
macro_rules! Depcrate_combinations_with_replacementcombinations_with_replacement {
() => {
// Module: crate::combinations_with_replacement
// Provides: {"combinations_with_replacement"}
// Dependencies: {}
# [doc = " Create a new `CombinationsWithReplacement` from a cloneable iterator."] pub fn combinations_with_replacement < I > (iter : I , k : usize) -> CombinationsWithReplacement < I > where I : Iterator , I :: Item : Clone , { let indices = alloc :: vec ! [0 ; k] . into_boxed_slice () ; CombinationsWithReplacementGeneric :: new (iter , indices) }
};
}
