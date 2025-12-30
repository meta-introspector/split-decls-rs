// Generated macro for GroupByDsl (trait)
macro_rules! Depcrate_query_dsl_group_by_dslGroupByDsl {
() => {
// Module: crate::query_dsl::group_by_dsl
// Provides: {"GroupByDsl"}
// Dependencies: {}
# [doc = " The `group_by` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `group_by` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait GroupByDsl < Expr : Expression > { # [doc = " The type returned by `.group_by`"] type Output ; # [doc = " See the trait documentation."] fn group_by (self , expr : Expr) -> dsl :: GroupBy < Self , Expr > ; }
};
}
