// Generated macro for impl_11 (impl)
macro_rules! Depcrate_mapimpl_11 {
() => {
// Module: crate::map
// Provides: {"impl_11"}
// Dependencies: {}
impl < T , V > Extend < (Idx < V > , T) > for ArenaMap < Idx < V > , T > { fn extend < I : IntoIterator < Item = (Idx < V > , T) > > (& mut self , iter : I) { iter . into_iter () . for_each (move | (k , v) | { self . insert (k , v) ; }) ; } }
};
}
