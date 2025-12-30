// Generated macro for macro_1054 (macro)
macro_rules! Depcrate_query_builder_limit_clausemacro_1054 {
() => {
// Module: crate::query_builder::limit_clause
// Provides: {"macro_1054"}
// Dependencies: {}
simple_clause ! (# [doc = " A query node indicating the absence of a limit clause"] # [doc = ""] # [doc = " This type is only relevant for implementing custom backends"] # [cfg_attr (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))] NoLimitClause , # [doc = " A query node representing a limit clause"] # [doc = ""] # [doc = " This type is only relevant for implementing custom backends"] # [cfg_attr (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))] LimitClause , " LIMIT ") ;
};
}
