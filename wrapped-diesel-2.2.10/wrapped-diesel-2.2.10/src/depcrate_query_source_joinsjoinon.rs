// Generated macro for JoinOn (struct)
macro_rules! Depcrate_query_source_joinsJoinOn {
() => {
// Module: crate::query_source::joins
// Provides: {"JoinOn"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , QueryId)] # [doc (hidden)] # [doc = " A query source representing the join between two tables with an explicit"] # [doc = " `ON` given. `Join` should usually be referenced instead, as all \"type"] # [doc = " safety\" traits are implemented in terms of `Join` implementing them."] pub struct JoinOn < Join , On > { join : Join , on : On , }
};
}
