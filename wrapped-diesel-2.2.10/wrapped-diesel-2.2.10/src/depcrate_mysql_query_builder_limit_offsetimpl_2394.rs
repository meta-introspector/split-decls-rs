// Generated macro for impl_2394 (impl)
macro_rules! Depcrate_mysql_query_builder_limit_offsetimpl_2394 {
() => {
// Module: crate::mysql::query_builder::limit_offset
// Provides: {"impl_2394"}
// Dependencies: {}
impl < 'a , L > IntoBoxedClause < 'a , Mysql > for LimitOffsetClause < LimitClause < L > , NoOffsetClause > where L : QueryFragment < Mysql > + Send + 'a , { type BoxedClause = BoxedLimitOffsetClause < 'a , Mysql > ; fn into_boxed (self) -> Self :: BoxedClause { BoxedLimitOffsetClause { limit : Some (Box :: new (self . limit_clause)) , offset : None , } } }
};
}
