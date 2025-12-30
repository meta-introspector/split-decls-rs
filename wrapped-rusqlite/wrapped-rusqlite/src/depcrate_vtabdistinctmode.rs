// Generated macro for DistinctMode (enum)
macro_rules! Depcrate_vtabDistinctMode {
() => {
// Module: crate::vtab
// Provides: {"DistinctMode"}
// Dependencies: {}
# [doc = " Determine if a virtual table query is DISTINCT"] # [non_exhaustive] # [derive (Debug , Eq , PartialEq)] pub enum DistinctMode { # [doc = " This is the default expectation."] Ordered , # [doc = " This mode is used when the query planner is doing a GROUP BY."] Grouped , # [doc = " This mode is used for a DISTINCT query."] Distinct , # [doc = " This mode is used for queries that have both DISTINCT and ORDER BY clauses."] DistinctOrdered , }
};
}
