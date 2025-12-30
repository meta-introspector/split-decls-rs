// Generated macro for impl_1176 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1176 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1176"}
// Dependencies: {}
impl < ST , QS , DB , GB > LimitDsl for BoxedSelectStatement < '_ , ST , QS , DB , GB > where DB : Backend , LimitClause < AsExprOf < i64 , BigInt > > : QueryFragment < DB > , { type Output = Self ; fn limit (mut self , limit : i64) -> Self :: Output { self . limit_offset . limit = Some (Box :: new (LimitClause (limit . into_sql :: < BigInt > ()))) ; self } }
};
}
