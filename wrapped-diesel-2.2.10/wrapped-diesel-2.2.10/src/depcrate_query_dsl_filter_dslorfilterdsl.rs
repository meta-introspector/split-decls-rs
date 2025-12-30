// Generated macro for OrFilterDsl (trait)
macro_rules! Depcrate_query_dsl_filter_dslOrFilterDsl {
() => {
// Module: crate::query_dsl::filter_dsl
// Provides: {"OrFilterDsl"}
// Dependencies: {}
# [doc = " The `or_filter` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `or_filter` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait OrFilterDsl < Predicate > { # [doc = " The type returned by `.filter`."] type Output ; # [doc = " See the trait documentation."] fn or_filter (self , predicate : Predicate) -> Self :: Output ; }
};
}
