// Generated macro for impl_1004 (impl)
macro_rules! Depcrate_renumberimpl_1004 {
() => {
// Module: crate::renumber
// Provides: {"impl_1004"}
// Dependencies: {}
impl RegionCtxt { # [doc = " Used to determine the representative of a component in the strongly connected"] # [doc = " constraint graph"] pub (crate) fn preference_value (self) -> usize { match self { RegionCtxt :: Unknown => 1 , RegionCtxt :: Existential (None) => 2 , RegionCtxt :: Existential (Some (_)) | RegionCtxt :: Free (_) => 2 , RegionCtxt :: Location (_) => 3 , RegionCtxt :: TyContext (_) => 4 , _ => 5 , } } }
};
}
