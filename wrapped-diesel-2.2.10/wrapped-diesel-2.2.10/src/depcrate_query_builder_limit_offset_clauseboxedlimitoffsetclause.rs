// Generated macro for BoxedLimitOffsetClause (struct)
macro_rules! Depcrate_query_builder_limit_offset_clauseBoxedLimitOffsetClause {
() => {
// Module: crate::query_builder::limit_offset_clause
// Provides: {"BoxedLimitOffsetClause"}
// Dependencies: {}
# [doc = " A boxed variant of [`LimitOffsetClause`](LimitOffsetClause)"] # [doc = ""] # [doc = " This type is only relevant for implementing custom backends"] # [allow (missing_debug_implementations)] # [cfg_attr (docsrs , doc (cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")))] pub struct BoxedLimitOffsetClause < 'a , DB > { # [doc = " The limit clause"] pub limit : Option < Box < dyn QueryFragment < DB > + Send + 'a > > , # [doc = " The offset clause"] pub offset : Option < Box < dyn QueryFragment < DB > + Send + 'a > > , }
};
}
