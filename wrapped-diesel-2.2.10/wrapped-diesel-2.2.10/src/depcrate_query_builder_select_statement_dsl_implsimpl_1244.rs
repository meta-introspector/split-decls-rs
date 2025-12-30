// Generated macro for impl_1244 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1244 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1244"}
// Dependencies: {}
impl < F , W > IntoUpdateTarget for SelectStatement < FromClause < F > , DefaultSelectClause < FromClause < F > > , NoDistinctClause , W > where F : QuerySource , Self : HasTable , W : ValidWhereClause < F > , { type WhereClause = W ; fn into_update_target (self) -> UpdateTarget < Self :: Table , Self :: WhereClause > { UpdateTarget { table : Self :: table () , where_clause : self . where_clause , } } }
};
}
