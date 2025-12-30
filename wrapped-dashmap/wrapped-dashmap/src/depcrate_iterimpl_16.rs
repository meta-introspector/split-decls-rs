// Generated macro for impl_16 (impl)
macro_rules! Depcrate_iterimpl_16 {
() => {
// Module: crate::iter
// Provides: {"impl_16"}
// Dependencies: {}
impl < K : Eq + Hash , V > Iterator for OwningIter < K , V > { type Item = (K , V) ; fn next (& mut self) -> Option < Self :: Item > { loop { if let Some (current) = self . current . as_mut () { if let Some ((k , v)) = current . next () { return Some ((k , v)) ; } } let iter = self . shards . next () ? . into_inner () . into_inner () . into_iter () ; self . current = Some (iter) ; } } }
};
}
