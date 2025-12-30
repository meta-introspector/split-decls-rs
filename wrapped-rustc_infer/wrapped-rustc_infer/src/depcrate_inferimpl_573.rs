// Generated macro for impl_573 (impl)
macro_rules! Depcrate_inferimpl_573 {
() => {
// Module: crate::infer
// Provides: {"impl_573"}
// Dependencies: {}
impl RegionVariableOrigin { pub fn span (& self) -> Span { match * self { RegionVariableOrigin :: Misc (a) | RegionVariableOrigin :: PatternRegion (a) | RegionVariableOrigin :: BorrowRegion (a) | RegionVariableOrigin :: Autoref (a) | RegionVariableOrigin :: Coercion (a) | RegionVariableOrigin :: RegionParameterDefinition (a , ..) | RegionVariableOrigin :: BoundRegion (a , ..) | RegionVariableOrigin :: UpvarRegion (_ , a) => a , RegionVariableOrigin :: Nll (..) => bug ! ("NLL variable used with `span`") , } } }
};
}
