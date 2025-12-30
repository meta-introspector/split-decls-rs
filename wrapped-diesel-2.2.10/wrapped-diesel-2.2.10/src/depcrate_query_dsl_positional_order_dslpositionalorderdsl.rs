// Generated macro for PositionalOrderDsl (trait)
macro_rules! Depcrate_query_dsl_positional_order_dslPositionalOrderDsl {
() => {
// Module: crate::query_dsl::positional_order_dsl
// Provides: {"PositionalOrderDsl"}
// Dependencies: {}
# [doc = " This trait is not yet part of Diesel's public API. It may change in the"] # [doc = " future without a major version bump."] # [doc = ""] # [doc = " This trait exists as a stop-gap for users who need to order by column position"] # [doc = " in their queries, so that they are not forced to drop entirely to raw SQL. The"] # [doc = " arguments to `positional_order_by` are not checked, nor is the select statement"] # [doc = " forced to be valid."] pub trait PositionalOrderDsl < Expr : Order > : Sized { fn positional_order_by (self , expr : Expr) -> PositionalOrderClause < Self , Expr :: Fragment > { PositionalOrderClause { source : self , expr : expr . into_fragment () , } } }
};
}
