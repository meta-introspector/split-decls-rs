// Generated macro for combinations (function)
macro_rules! Depcrate_combinationscombinations {
() => {
// Module: crate::combinations
// Provides: {"combinations"}
// Dependencies: {}
# [doc = " Create a new `Combinations` from a cloneable iterator."] pub fn combinations < I : Iterator > (iter : I , k : usize) -> Combinations < I > where I :: Item : Clone , { Combinations :: new (iter , (0 .. k) . collect ()) }
};
}
