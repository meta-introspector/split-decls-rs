// Generated macro for HavingDsl (trait)
macro_rules! Depcrate_query_dsl_having_dslHavingDsl {
() => {
// Module: crate::query_dsl::having_dsl
// Provides: {"HavingDsl"}
// Dependencies: {}
# [doc = " The `having` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `having` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait HavingDsl < Predicate > { # [doc = " The type returned by `.having`."] type Output ; # [doc = " See the trait documentation."] fn having (self , predicate : Predicate) -> dsl :: Having < Self , Predicate > ; }
};
}
