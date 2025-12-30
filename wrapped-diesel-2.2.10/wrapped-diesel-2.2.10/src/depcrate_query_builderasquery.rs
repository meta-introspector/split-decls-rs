// Generated macro for AsQuery (trait)
macro_rules! Depcrate_query_builderAsQuery {
() => {
// Module: crate::query_builder
// Provides: {"AsQuery"}
// Dependencies: {}
# [doc = " Types that can be converted into a complete, typed SQL query."] # [doc = ""] # [doc = " This is used internally to automatically add the right select clause when"] # [doc = " none is specified, or to automatically add `RETURNING *` in certain contexts."] # [doc = ""] # [doc = " A type which implements this trait is guaranteed to be valid for execution."] pub trait AsQuery { # [doc = " The SQL type of `Self::Query`"] type SqlType ; # [doc = " What kind of query does this type represent?"] type Query : Query < SqlType = Self :: SqlType > ; # [doc = " Converts a type which semantically represents a SQL query into the"] # [doc = " actual query being executed. See the trait level docs for more."] # [allow (clippy :: wrong_self_convention)] fn as_query (self) -> Self :: Query ; }
};
}
