// Generated macro for impl_8 (impl)
macro_rules! Depcrate_mapimpl_8 {
() => {
// Module: crate::map
// Provides: {"impl_8"}
// Dependencies: {}
impl < T , V > std :: ops :: Index < Idx < V > > for ArenaMap < Idx < V > , T > { type Output = T ; fn index (& self , idx : Idx < V >) -> & T { self . v [Self :: to_idx (idx)] . as_ref () . unwrap () } }
};
}
