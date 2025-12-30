// Generated macro for impl_3493 (impl)
macro_rules! Depcrate_pg_query_builder_limit_offsetimpl_3493 {
() => {
// Module: crate::pg::query_builder::limit_offset
// Provides: {"impl_3493"}
// Dependencies: {}
impl < 'a , L , O > IntoBoxedClause < 'a , Pg > for LimitOffsetClause < L , O > where L : QueryFragment < Pg > + Send + 'a , O : QueryFragment < Pg > + Send + 'a , { type BoxedClause = BoxedLimitOffsetClause < 'a , Pg > ; fn into_boxed (self) -> Self :: BoxedClause { BoxedLimitOffsetClause { limit : Some (Box :: new (self . limit_clause)) , offset : Some (Box :: new (self . offset_clause)) , } } }
};
}
