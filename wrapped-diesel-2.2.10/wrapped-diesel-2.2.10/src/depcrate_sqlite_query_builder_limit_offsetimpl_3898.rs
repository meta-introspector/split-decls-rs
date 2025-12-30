// Generated macro for impl_3898 (impl)
macro_rules! Depcrate_sqlite_query_builder_limit_offsetimpl_3898 {
() => {
// Module: crate::sqlite::query_builder::limit_offset
// Provides: {"impl_3898"}
// Dependencies: {}
impl < 'a , L > IntoBoxedClause < 'a , Sqlite > for LimitOffsetClause < LimitClause < L > , NoOffsetClause > where L : QueryFragment < Sqlite > + Send + 'a , { type BoxedClause = BoxedLimitOffsetClause < 'a , Sqlite > ; fn into_boxed (self) -> Self :: BoxedClause { BoxedLimitOffsetClause { limit : Some (Box :: new (self . limit_clause)) , offset : None , } } }
};
}
