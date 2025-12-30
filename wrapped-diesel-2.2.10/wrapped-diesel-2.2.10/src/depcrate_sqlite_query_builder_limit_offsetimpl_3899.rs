// Generated macro for impl_3899 (impl)
macro_rules! Depcrate_sqlite_query_builder_limit_offsetimpl_3899 {
() => {
// Module: crate::sqlite::query_builder::limit_offset
// Provides: {"impl_3899"}
// Dependencies: {}
impl < 'a , O > IntoBoxedClause < 'a , Sqlite > for LimitOffsetClause < NoLimitClause , OffsetClause < O > > where O : QueryFragment < Sqlite > + Send + 'a , { type BoxedClause = BoxedLimitOffsetClause < 'a , Sqlite > ; fn into_boxed (self) -> Self :: BoxedClause { BoxedLimitOffsetClause { limit : None , offset : Some (Box :: new (self . offset_clause)) , } } }
};
}
