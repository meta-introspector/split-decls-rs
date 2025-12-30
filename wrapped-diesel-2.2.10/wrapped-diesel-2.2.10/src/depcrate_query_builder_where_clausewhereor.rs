// Generated macro for WhereOr (trait)
macro_rules! Depcrate_query_builder_where_clauseWhereOr {
() => {
// Module: crate::query_builder::where_clause
// Provides: {"WhereOr"}
// Dependencies: {}
# [doc = " Add `Predicate` to the current `WHERE` clause, joining with `OR` if"] # [doc = " applicable."] pub trait WhereOr < Predicate > { # [doc = " What is the type of the resulting `WHERE` clause?"] type Output ; # [doc = " See the trait-level docs."] fn or (self , predicate : Predicate) -> Self :: Output ; }
};
}
