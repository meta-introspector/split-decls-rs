// Generated macro for macro_19 (macro)
macro_rules! Depcrate_merge_basemacro_19 {
() => {
// Module: crate::merge_base
// Provides: {"macro_19"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " The flags used in the graph for finding [merge bases](crate::merge_base())."] # [derive (Debug , Default , Copy , Clone , Eq , PartialEq)] pub struct Flags : u8 { # [doc = " The commit belongs to the graph reachable by the first commit"] const COMMIT1 = 1 << 0 ; # [doc = " The commit belongs to the graph reachable by all other commits."] const COMMIT2 = 1 << 1 ; # [doc = " Marks the commit as done, it's reachable by both COMMIT1 and COMMIT2."] const STALE = 1 << 2 ; # [doc = " The commit was already put ontto the results list."] const RESULT = 1 << 3 ; } }
};
}
