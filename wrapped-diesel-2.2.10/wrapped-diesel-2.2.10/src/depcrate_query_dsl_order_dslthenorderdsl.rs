// Generated macro for ThenOrderDsl (trait)
macro_rules! Depcrate_query_dsl_order_dslThenOrderDsl {
() => {
// Module: crate::query_dsl::order_dsl
// Provides: {"ThenOrderDsl"}
// Dependencies: {}
# [doc = " The `then_order_by` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `then_order_by` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait ThenOrderDsl < Expr > { # [doc = " The type returned by `.then_order_by`."] type Output ; # [doc = " See the trait documentation."] fn then_order_by (self , expr : Expr) -> Self :: Output ; }
};
}
