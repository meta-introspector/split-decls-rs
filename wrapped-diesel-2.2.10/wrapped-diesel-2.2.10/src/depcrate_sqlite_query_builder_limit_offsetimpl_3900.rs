// Generated macro for impl_3900 (impl)
macro_rules! Depcrate_sqlite_query_builder_limit_offsetimpl_3900 {
() => {
// Module: crate::sqlite::query_builder::limit_offset
// Provides: {"impl_3900"}
// Dependencies: {}
impl < 'a , L , O > IntoBoxedClause < 'a , Sqlite > for LimitOffsetClause < LimitClause < L > , OffsetClause < O > > where L : QueryFragment < Sqlite > + Send + 'a , O : QueryFragment < Sqlite > + Send + 'a , { type BoxedClause = BoxedLimitOffsetClause < 'a , Sqlite > ; fn into_boxed (self) -> Self :: BoxedClause { BoxedLimitOffsetClause { limit : Some (Box :: new (self . limit_clause)) , offset : Some (Box :: new (self . offset_clause)) , } } }
};
}
