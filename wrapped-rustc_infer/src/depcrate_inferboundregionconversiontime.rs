// Generated macro for BoundRegionConversionTime (enum)
macro_rules! Depcrate_inferBoundRegionConversionTime {
() => {
// Module: crate::infer
// Provides: {"BoundRegionConversionTime"}
// Dependencies: {}
# [doc = " Times when we replace bound regions with existentials:"] # [derive (Clone , Copy , Debug)] pub enum BoundRegionConversionTime { # [doc = " when a fn is called"] FnCall , # [doc = " when two higher-ranked types are compared"] HigherRankedType , # [doc = " when projecting an associated type"] AssocTypeProjection (DefId) , }
};
}
