// Generated macro for MirLocalUsage (enum)
macro_rules! Depcrate_redundant_cloneMirLocalUsage {
() => {
// Module: crate::redundant_clone
// Provides: {"MirLocalUsage"}
// Dependencies: {}
# [derive (Debug , Default)] enum MirLocalUsage { # [doc = " The local maybe used, but we are not sure how."] Unknown , # [doc = " The local is not used."] # [default] Unused , # [doc = " The local is used at a specific location."] Used (mir :: Location) , }
};
}
