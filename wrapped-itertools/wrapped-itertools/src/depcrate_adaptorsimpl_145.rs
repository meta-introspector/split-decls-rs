// Generated macro for impl_145 (impl)
macro_rules! Depcrate_adaptorsimpl_145 {
() => {
// Module: crate::adaptors
// Provides: {"impl_145"}
// Dependencies: {}
impl < I , T > Iterator for TupleCombinations < I , T > where I : Iterator , T : HasCombination < I > , { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } fn size_hint (& self) -> SizeHint { self . iter . size_hint () } fn count (self) -> usize { self . iter . count () } fn fold < B , F > (self , init : B , f : F) -> B where F : FnMut (B , Self :: Item) -> B , { self . iter . fold (init , f) } }
};
}
