// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl < K , V > Iterator for IntoIter < K , V > where K : Hash + Eq , { type Item = (K , V) ; fn next (& mut self) -> Option < (K , V) > { self . cache . pop_lru () } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . cache . len () ; (len , Some (len)) } fn count (self) -> usize { self . cache . len () } }
};
}
