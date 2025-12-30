// Generated macro for impl_3897 (impl)
macro_rules! Depcrate_sqlite_query_builder_limit_offsetimpl_3897 {
() => {
// Module: crate::sqlite::query_builder::limit_offset
// Provides: {"impl_3897"}
// Dependencies: {}
impl < 'a > IntoBoxedClause < 'a , Sqlite > for LimitOffsetClause < NoLimitClause , NoOffsetClause > { type BoxedClause = BoxedLimitOffsetClause < 'a , Sqlite > ; fn into_boxed (self) -> Self :: BoxedClause { BoxedLimitOffsetClause { limit : None , offset : None , } } }
};
}
