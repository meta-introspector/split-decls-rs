// Generated macro for impl_1250 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1250 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1250"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H > SelectNullableDsl for SelectStatement < F , SelectClause < S > , D , W , O , LOf , G , H > { type Output = SelectStatement < F , SelectClause < Nullable < S > > , D , W , O , LOf , G , H > ; fn nullable (self) -> Self :: Output { SelectStatement :: new (SelectClause (Nullable :: new (self . select . 0)) , self . from , self . distinct , self . where_clause , self . order , self . limit_offset , self . group_by , self . having , self . locking ,) } }
};
}
