// Generated macro for OrderDsl (trait)
macro_rules! Depcrate_query_dsl_order_dslOrderDsl {
() => {
// Module: crate::query_dsl::order_dsl
// Provides: {"OrderDsl"}
// Dependencies: {}
# [doc = " The `order` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `order` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait OrderDsl < Expr : Expression > { # [doc = " The type returned by `.order`."] type Output ; # [doc = " See the trait documentation."] fn order (self , expr : Expr) -> Self :: Output ; }
};
}
