// Generated macro for WhereAnd (trait)
macro_rules! Depcrate_query_builder_where_clauseWhereAnd {
() => {
// Module: crate::query_builder::where_clause
// Provides: {"WhereAnd"}
// Dependencies: {}
# [doc = " Add `Predicate` to the current `WHERE` clause, joining with `AND` if"] # [doc = " applicable."] pub trait WhereAnd < Predicate > { # [doc = " What is the type of the resulting `WHERE` clause?"] type Output ; # [doc = " See the trait-level docs."] fn and (self , predicate : Predicate) -> Self :: Output ; }
};
}
