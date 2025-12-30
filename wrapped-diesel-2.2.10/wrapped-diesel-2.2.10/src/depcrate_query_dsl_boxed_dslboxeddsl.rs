// Generated macro for BoxedDsl (trait)
macro_rules! Depcrate_query_dsl_boxed_dslBoxedDsl {
() => {
// Module: crate::query_dsl::boxed_dsl
// Provides: {"BoxedDsl"}
// Dependencies: {}
# [doc = " The `into_boxed` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `into_boxed` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait BoxedDsl < 'a , DB > { # [doc = " The return type of `internal_into_boxed`"] type Output ; # [doc = " See the trait documentation."] fn internal_into_boxed (self) -> dsl :: IntoBoxed < 'a , Self , DB > ; }
};
}
