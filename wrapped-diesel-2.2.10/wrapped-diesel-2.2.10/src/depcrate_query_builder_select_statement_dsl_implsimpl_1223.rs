// Generated macro for impl_1223 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1223 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1223"}
// Dependencies: {}
impl < ST , F , S , D , W , O , LOf , G , H > DistinctDsl for SelectStatement < F , S , D , W , O , LOf , G , H > where Self : SelectQuery < SqlType = ST > , SelectStatement < F , S , DistinctClause , W , O , LOf , G , H > : SelectQuery < SqlType = ST > , { type Output = SelectStatement < F , S , DistinctClause , W , O , LOf , G , H > ; fn distinct (self) -> Self :: Output { SelectStatement :: new (self . select , self . from , DistinctClause , self . where_clause , self . order , self . limit_offset , self . group_by , self . having , self . locking ,) } }
};
}
