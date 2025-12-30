// Generated macro for impl_863 (impl)
macro_rules! Depcrate_validation_rules_overlapping_fields_can_be_mergedimpl_863 {
() => {
// Module: crate::validation::rules::overlapping_fields_can_be_merged
// Provides: {"impl_863"}
// Dependencies: {}
impl < 'a , K : Eq + Hash + 'a , V : 'a > Iterator for OrderedMapIter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . and_then (| key | self . map . get (key) . map (| value | (key , value))) } }
};
}
