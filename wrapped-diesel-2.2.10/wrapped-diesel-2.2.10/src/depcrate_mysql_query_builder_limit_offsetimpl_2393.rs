// Generated macro for impl_2393 (impl)
macro_rules! Depcrate_mysql_query_builder_limit_offsetimpl_2393 {
() => {
// Module: crate::mysql::query_builder::limit_offset
// Provides: {"impl_2393"}
// Dependencies: {}
impl < 'a > IntoBoxedClause < 'a , Mysql > for LimitOffsetClause < NoLimitClause , NoOffsetClause > { type BoxedClause = BoxedLimitOffsetClause < 'a , Mysql > ; fn into_boxed (self) -> Self :: BoxedClause { BoxedLimitOffsetClause { limit : None , offset : None , } } }
};
}
