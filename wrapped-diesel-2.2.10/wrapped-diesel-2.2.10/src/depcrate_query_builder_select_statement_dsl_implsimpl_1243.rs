// Generated macro for impl_1243 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1243 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1243"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC > HasTable for SelectStatement < FromClause < F > , S , D , W , O , LOf , G , H , LC > where F : HasTable + QuerySource , { type Table = F :: Table ; fn table () -> Self :: Table { F :: table () } }
};
}
