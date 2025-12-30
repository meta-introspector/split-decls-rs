// Generated macro for ReturningClause (struct)
macro_rules! Depcrate_query_builder_returning_clauseReturningClause {
() => {
// Module: crate::query_builder::returning_clause
// Provides: {"ReturningClause"}
// Dependencies: {}
# [doc = " This type represents a SQL `Returning` clause"] # [doc = ""] # [doc = " Custom backends can specialize the [`QueryFragment`]"] # [doc = " implementation via"] # [doc = " [`SqlDialect::ReturningClause`](crate::backend::SqlDialect::ReturningClause)"] # [cfg_attr (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))] # [derive (Debug , Clone , Copy , QueryId)] pub struct ReturningClause < Expr > (pub Expr) ;
};
}
