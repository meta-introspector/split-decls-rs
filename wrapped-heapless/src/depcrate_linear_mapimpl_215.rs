// Generated macro for impl_215 (impl)
macro_rules! Depcrate_linear_mapimpl_215 {
() => {
// Module: crate::linear_map
// Provides: {"impl_215"}
// Dependencies: {}
impl < K , V , const N : usize > Iterator for IntoIter < K , V , N > where K : Eq , { type Item = (K , V) ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
};
}
