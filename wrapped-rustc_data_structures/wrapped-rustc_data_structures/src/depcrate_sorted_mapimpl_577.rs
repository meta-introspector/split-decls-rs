// Generated macro for impl_577 (impl)
macro_rules! Depcrate_sorted_mapimpl_577 {
() => {
// Module: crate::sorted_map
// Provides: {"impl_577"}
// Dependencies: {}
impl < K : Debug , V : Debug > Debug for SortedMap < K , V > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_map () . entries (self . data . iter () . map (| (a , b) | (a , b))) . finish () } }
};
}
