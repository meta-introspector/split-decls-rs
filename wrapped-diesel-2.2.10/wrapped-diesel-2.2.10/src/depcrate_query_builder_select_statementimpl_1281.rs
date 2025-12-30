// Generated macro for impl_1281 (impl)
macro_rules! Depcrate_query_builder_select_statementimpl_1281 {
() => {
// Module: crate::query_builder::select_statement
// Provides: {"impl_1281"}
// Dependencies: {}
impl < F : QuerySource > SelectStatement < FromClause < F > > { # [doc (hidden)] pub fn simple (from : F) -> Self { let from = FromClause :: new (from) ; SelectStatement :: new (DefaultSelectClause :: new (& from) , from , NoDistinctClause , NoWhereClause , NoOrderClause , LimitOffsetClause { limit_clause : NoLimitClause , offset_clause : NoOffsetClause , } , NoGroupByClause , NoHavingClause , NoLockingClause ,) } }
};
}
