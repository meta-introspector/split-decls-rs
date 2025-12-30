// Generated macro for FindDsl (trait)
macro_rules! Depcrate_query_dsl_filter_dslFindDsl {
() => {
// Module: crate::query_dsl::filter_dsl
// Provides: {"FindDsl"}
// Dependencies: {}
# [doc = " The `find` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `find` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait FindDsl < PK > { # [doc = " The type returned by `.find`."] type Output ; # [doc = " See the trait documentation."] fn find (self , id : PK) -> Self :: Output ; }
};
}
