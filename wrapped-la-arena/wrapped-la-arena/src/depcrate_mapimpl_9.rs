// Generated macro for impl_9 (impl)
macro_rules! Depcrate_mapimpl_9 {
() => {
// Module: crate::map
// Provides: {"impl_9"}
// Dependencies: {}
impl < T , V > std :: ops :: IndexMut < Idx < V > > for ArenaMap < Idx < V > , T > { fn index_mut (& mut self , idx : Idx < V >) -> & mut T { self . v [Self :: to_idx (idx)] . as_mut () . unwrap () } }
};
}
