// Generated macro for Query (trait)
macro_rules! Depcrate_query_builderQuery {
() => {
// Module: crate::query_builder
// Provides: {"Query"}
// Dependencies: {}
# [doc = " A complete SQL query with a return type."] # [doc = ""] # [doc = " This can be a select statement, or a command such as `update` or `insert`"] # [doc = " with a `RETURNING` clause. Unlike [`Expression`], types implementing this"] # [doc = " trait are guaranteed to be executable on their own."] # [doc = ""] # [doc = " A type which doesn't implement this trait may still represent a complete SQL"] # [doc = " query. For example, an `INSERT` statement without a `RETURNING` clause will"] # [doc = " not implement this trait, but can still be executed."] # [doc = ""] # [doc = " [`Expression`]: crate::expression::Expression"] pub trait Query { # [doc = " The SQL type that this query represents."] # [doc = ""] # [doc = " This is the SQL type of the `SELECT` clause for select statements, and"] # [doc = " the SQL type of the `RETURNING` clause for insert, update, or delete"] # [doc = " statements."] type SqlType ; }
};
}
