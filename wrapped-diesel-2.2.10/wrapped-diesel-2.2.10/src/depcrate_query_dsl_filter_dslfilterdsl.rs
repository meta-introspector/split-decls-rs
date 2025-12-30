// Generated macro for FilterDsl (trait)
macro_rules! Depcrate_query_dsl_filter_dslFilterDsl {
() => {
// Module: crate::query_dsl::filter_dsl
// Provides: {"FilterDsl"}
// Dependencies: {}
# [doc = " The `filter` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `filter` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait FilterDsl < Predicate > { # [doc = " The type returned by `.filter`."] type Output ; # [doc = " See the trait documentation."] fn filter (self , predicate : Predicate) -> Self :: Output ; }
};
}
