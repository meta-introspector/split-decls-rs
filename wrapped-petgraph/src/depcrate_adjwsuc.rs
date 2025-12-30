// Generated macro for WSuc (struct)
macro_rules! Depcrate_adjWSuc {
() => {
// Module: crate::adj
// Provides: {"WSuc"}
// Dependencies: {}
# [doc = " Weighted successor"] # [derive (Clone , Debug , Hash , PartialEq , Eq , PartialOrd , Ord)] struct WSuc < E , Ix : IndexType > { # [doc = " Index of the successor."] suc : Ix , # [doc = " Weight of the edge to `suc`."] weight : E , }
};
}
