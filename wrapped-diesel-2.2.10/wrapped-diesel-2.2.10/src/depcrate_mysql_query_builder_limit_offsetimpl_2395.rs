// Generated macro for impl_2395 (impl)
macro_rules! Depcrate_mysql_query_builder_limit_offsetimpl_2395 {
() => {
// Module: crate::mysql::query_builder::limit_offset
// Provides: {"impl_2395"}
// Dependencies: {}
impl < 'a , L , O > IntoBoxedClause < 'a , Mysql > for LimitOffsetClause < LimitClause < L > , OffsetClause < O > > where L : QueryFragment < Mysql > + Send + 'a , O : QueryFragment < Mysql > + Send + 'a , { type BoxedClause = BoxedLimitOffsetClause < 'a , Mysql > ; fn into_boxed (self) -> Self :: BoxedClause { BoxedLimitOffsetClause { limit : Some (Box :: new (self . limit_clause)) , offset : Some (Box :: new (self . offset_clause)) , } } }
};
}
