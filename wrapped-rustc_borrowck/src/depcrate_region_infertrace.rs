// Generated macro for Trace (enum)
macro_rules! Depcrate_region_inferTrace {
() => {
// Module: crate::region_infer
// Provides: {"Trace"}
// Dependencies: {}
# [derive (Clone , PartialEq , Eq , Debug)] enum Trace < 'a , 'tcx > { StartRegion , FromGraph (& 'a OutlivesConstraint < 'tcx >) , FromStatic (RegionVid) , NotVisited , }
};
}
