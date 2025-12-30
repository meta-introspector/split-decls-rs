// Generated macro for SelectDsl (trait)
macro_rules! Depcrate_query_dsl_select_dslSelectDsl {
() => {
// Module: crate::query_dsl::select_dsl
// Provides: {"SelectDsl"}
// Dependencies: {}
# [doc = " The `select` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `select` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait SelectDsl < Selection : Expression > { # [doc = " The type returned by `.select`"] type Output ; # [doc = " See the trait documentation"] fn select (self , selection : Selection) -> Self :: Output ; }
};
}
