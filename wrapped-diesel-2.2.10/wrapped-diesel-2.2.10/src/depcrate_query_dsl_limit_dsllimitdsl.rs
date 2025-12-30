// Generated macro for LimitDsl (trait)
macro_rules! Depcrate_query_dsl_limit_dslLimitDsl {
() => {
// Module: crate::query_dsl::limit_dsl
// Provides: {"LimitDsl"}
// Dependencies: {}
# [doc = " The `limit` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `limit` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait LimitDsl < DummyArgForAutoType = i64 > { # [doc = " The type returned by `.limit`"] type Output ; # [doc = " See the trait documentation"] fn limit (self , limit : i64) -> Self :: Output ; }
};
}
