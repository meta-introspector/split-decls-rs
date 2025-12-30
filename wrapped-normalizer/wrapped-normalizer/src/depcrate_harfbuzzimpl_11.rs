// Generated macro for impl_11 (impl)
macro_rules! Depcrate_harfbuzzimpl_11 {
() => {
// Module: crate::harfbuzz
// Provides: {"impl_11"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl DecomposeFunc for CanonicalDecompositionBorrowed < '_ > { fn decompose (& self , ab : char) -> Option < (char , char) > { match CanonicalDecompositionBorrowed :: decompose (self , ab) { Decomposed :: Default => None , Decomposed :: Expansion (first , second) => Some ((first , second)) , Decomposed :: Singleton (single) => Some ((single , '\0')) , } } }
};
}
