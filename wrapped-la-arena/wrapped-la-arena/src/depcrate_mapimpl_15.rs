// Generated macro for impl_15 (impl)
macro_rules! Depcrate_mapimpl_15 {
() => {
// Module: crate::map
// Provides: {"impl_15"}
// Dependencies: {}
impl < T , V > ArenaMapIter < Idx < T > , V > { fn mapper ((idx , o) : (usize , Option < V >)) -> Option < (Idx < T > , V) > { Some ((ArenaMap :: < Idx < T > , V > :: from_idx (idx) , o ?)) } }
};
}
