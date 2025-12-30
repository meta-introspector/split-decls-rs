// Generated macro for SelectQuery (trait)
macro_rules! Depcrate_query_builderSelectQuery {
() => {
// Module: crate::query_builder
// Provides: {"SelectQuery"}
// Dependencies: {}
# [doc = " Indicates that a type is a `SELECT` statement."] # [doc = ""] # [doc = " This trait differs from `Query` in two ways:"] # [doc = " - It is implemented only for select statements, rather than all queries"] # [doc = "   which return a value."] # [doc = " - It has looser constraints. A type implementing `SelectQuery` is known to"] # [doc = "   be potentially valid if used as a subselect, but it is not necessarily"] # [doc = "   able to be executed."] pub trait SelectQuery { # [doc = " The SQL type of the `SELECT` clause"] type SqlType ; }
};
}
