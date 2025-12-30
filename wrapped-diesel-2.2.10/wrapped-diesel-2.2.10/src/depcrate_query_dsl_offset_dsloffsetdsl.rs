// Generated macro for OffsetDsl (trait)
macro_rules! Depcrate_query_dsl_offset_dslOffsetDsl {
() => {
// Module: crate::query_dsl::offset_dsl
// Provides: {"OffsetDsl"}
// Dependencies: {}
# [doc = " The `offset` method"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `offset` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait OffsetDsl < DummyArgForAutoType = i64 > { # [doc = " The type returned by `.offset`."] type Output ; # [doc = " See the trait documentation"] fn offset (self , offset : i64) -> Self :: Output ; }
};
}
