// Generated macro for impl_12 (impl)
macro_rules! Depcrate_mapimpl_12 {
() => {
// Module: crate::map
// Provides: {"impl_12"}
// Dependencies: {}
impl < T , V > FromIterator < (Idx < V > , T) > for ArenaMap < Idx < V > , T > { fn from_iter < I : IntoIterator < Item = (Idx < V > , T) > > (iter : I) -> Self { let mut this = Self :: new () ; this . extend (iter) ; this } }
};
}
