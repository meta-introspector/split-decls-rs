// Generated macro for DistinctDsl (trait)
macro_rules! Depcrate_query_dsl_distinct_dslDistinctDsl {
() => {
// Module: crate::query_dsl::distinct_dsl
// Provides: {"DistinctDsl"}
// Dependencies: {}
# [doc = " The `distinct` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `distinct` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait DistinctDsl { # [doc = " The type returned by `.distinct`"] type Output ; # [doc = " See the trait documentation."] fn distinct (self) -> dsl :: Distinct < Self > ; }
};
}
