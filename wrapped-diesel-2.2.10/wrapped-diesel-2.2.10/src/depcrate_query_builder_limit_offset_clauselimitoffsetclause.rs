// Generated macro for LimitOffsetClause (struct)
macro_rules! Depcrate_query_builder_limit_offset_clauseLimitOffsetClause {
() => {
// Module: crate::query_builder::limit_offset_clause
// Provides: {"LimitOffsetClause"}
// Dependencies: {}
# [doc = " A helper query node that contains both limit and offset clauses"] # [doc = ""] # [doc = " This type is only relevant for implementing custom backends"] # [derive (Debug , Clone , Copy , QueryId)] # [cfg_attr (docsrs , doc (cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")))] pub struct LimitOffsetClause < Limit , Offset > { # [doc = " The limit clause"] pub limit_clause : Limit , # [doc = " The offset clause"] pub offset_clause : Offset , }
};
}
