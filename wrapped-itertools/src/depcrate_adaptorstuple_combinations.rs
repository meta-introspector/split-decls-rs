// Generated macro for tuple_combinations (function)
macro_rules! Depcrate_adaptorstuple_combinations {
() => {
// Module: crate::adaptors
// Provides: {"tuple_combinations"}
// Dependencies: {}
# [doc = " Create a new `TupleCombinations` from a cloneable iterator."] pub fn tuple_combinations < T , I > (iter : I) -> TupleCombinations < I , T > where I : Iterator , I :: Item : Clone , T : HasCombination < I > , { TupleCombinations { iter : T :: Combination :: from (iter) , _mi : PhantomData , } }
};
}
