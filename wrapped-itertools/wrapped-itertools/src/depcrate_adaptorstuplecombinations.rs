// Generated macro for TupleCombinations (struct)
macro_rules! Depcrate_adaptorsTupleCombinations {
() => {
// Module: crate::adaptors
// Provides: {"TupleCombinations"}
// Dependencies: {}
# [doc = " An iterator to iterate through all combinations in a `Clone`-able iterator that produces tuples"] # [doc = " of a specific size."] # [doc = ""] # [doc = " See [`.tuple_combinations()`](crate::Itertools::tuple_combinations) for more"] # [doc = " information."] # [derive (Clone , Debug)] # [must_use = "this iterator adaptor is not lazy but does nearly nothing unless consumed"] pub struct TupleCombinations < I , T > where I : Iterator , T : HasCombination < I > , { iter : T :: Combination , _mi : PhantomData < I > , }
};
}
