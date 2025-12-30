// Generated macro for impl_982 (impl)
macro_rules! Depcrate_region_inferimpl_982 {
() => {
// Module: crate::region_infer
// Provides: {"impl_982"}
// Dependencies: {}
impl Representative { pub (crate) fn rvid (self) -> RegionVid { match self { Representative :: FreeRegion (region_vid) | Representative :: Placeholder (region_vid) | Representative :: Existential (region_vid) => region_vid , } } pub (crate) fn new (r : RegionVid , definition : & RegionDefinition < '_ >) -> Self { match definition . origin { NllRegionVariableOrigin :: FreeRegion => Representative :: FreeRegion (r) , NllRegionVariableOrigin :: Placeholder (_) => Representative :: Placeholder (r) , NllRegionVariableOrigin :: Existential { .. } => Representative :: Existential (r) , } } }
};
}
