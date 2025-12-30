// Generated macro for Representative (enum)
macro_rules! Depcrate_region_inferRepresentative {
() => {
// Module: crate::region_infer
// Provides: {"Representative"}
// Dependencies: {}
# [doc = " The representative region variable for an SCC, tagged by its origin."] # [doc = " We prefer placeholders over existentially quantified variables, otherwise"] # [doc = " it's the one with the smallest Region Variable ID. In other words,"] # [doc = " the order of this enumeration really matters!"] # [derive (Copy , Debug , Clone , PartialEq , PartialOrd , Eq , Ord)] pub (crate) enum Representative { FreeRegion (RegionVid) , Placeholder (RegionVid) , Existential (RegionVid) , }
};
}
