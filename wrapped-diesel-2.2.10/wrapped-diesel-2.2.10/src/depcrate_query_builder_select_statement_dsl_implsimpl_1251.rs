// Generated macro for impl_1251 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1251 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1251"}
// Dependencies: {}
impl < F , D , W , O , LOf , G , H > SelectNullableDsl for SelectStatement < F , DefaultSelectClause < F > , D , W , O , LOf , G , H > where F : AsQuerySource , { type Output = SelectStatement < F , SelectClause < Nullable < < F :: QuerySource as QuerySource > :: DefaultSelection > > , D , W , O , LOf , G , H , > ; fn nullable (self) -> Self :: Output { SelectStatement :: new (SelectClause (Nullable :: new (self . from . as_query_source () . default_selection () ,)) , self . from , self . distinct , self . where_clause , self . order , self . limit_offset , self . group_by , self . having , self . locking ,) } }
};
}
