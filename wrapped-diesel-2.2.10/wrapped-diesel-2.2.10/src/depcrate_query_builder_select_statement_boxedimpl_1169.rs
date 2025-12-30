// Generated macro for impl_1169 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1169 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1169"}
// Dependencies: {}
impl < ST , QS , DB , GB > DistinctDsl for BoxedSelectStatement < '_ , ST , QS , DB , GB > where DB : Backend , DistinctClause : QueryFragment < DB > , { type Output = Self ; fn distinct (mut self) -> Self :: Output { self . distinct = Box :: new (DistinctClause) ; self } }
};
}
