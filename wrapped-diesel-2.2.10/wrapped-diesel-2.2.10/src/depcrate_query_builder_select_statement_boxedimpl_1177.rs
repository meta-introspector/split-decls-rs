// Generated macro for impl_1177 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1177 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1177"}
// Dependencies: {}
impl < ST , QS , DB , GB > OffsetDsl for BoxedSelectStatement < '_ , ST , QS , DB , GB > where DB : Backend , OffsetClause < AsExprOf < i64 , BigInt > > : QueryFragment < DB > , { type Output = Self ; fn offset (mut self , offset : i64) -> Self :: Output { self . limit_offset . offset = Some (Box :: new (OffsetClause (offset . into_sql :: < BigInt > ()))) ; self } }
};
}
