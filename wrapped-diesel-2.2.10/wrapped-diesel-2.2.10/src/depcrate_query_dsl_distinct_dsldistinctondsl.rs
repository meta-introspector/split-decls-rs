// Generated macro for DistinctOnDsl (trait)
macro_rules! Depcrate_query_dsl_distinct_dslDistinctOnDsl {
() => {
// Module: crate::query_dsl::distinct_dsl
// Provides: {"DistinctOnDsl"}
// Dependencies: {}
# [doc = " The `distinct_on` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `distinct_on` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] # [cfg (feature = "postgres_backend")] pub trait DistinctOnDsl < Selection > { # [doc = " The type returned by `.distinct_on`"] type Output ; # [doc = " See the trait documentation"] fn distinct_on (self , selection : Selection) -> dsl :: DistinctOn < Self , Selection > ; }
};
}
