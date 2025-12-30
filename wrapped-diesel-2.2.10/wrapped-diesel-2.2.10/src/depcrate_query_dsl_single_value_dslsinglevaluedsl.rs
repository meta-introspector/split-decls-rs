// Generated macro for SingleValueDsl (trait)
macro_rules! Depcrate_query_dsl_single_value_dslSingleValueDsl {
() => {
// Module: crate::query_dsl::single_value_dsl
// Provides: {"SingleValueDsl"}
// Dependencies: {}
# [doc = " The `single_value` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `single_value` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait SingleValueDsl { # [doc = " The type returned by `.single_value`."] type Output ; # [doc = " See the trait documentation."] fn single_value (self) -> Self :: Output ; }
};
}
