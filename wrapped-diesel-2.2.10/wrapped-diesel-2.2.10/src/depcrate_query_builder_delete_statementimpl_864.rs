// Generated macro for impl_864 (impl)
macro_rules! Depcrate_query_builder_delete_statementimpl_864 {
() => {
// Module: crate::query_builder::delete_statement
// Provides: {"impl_864"}
// Dependencies: {}
impl < 'a , T , U , Ret , DB > BoxedDsl < 'a , DB > for DeleteStatement < T , U , Ret > where U : Into < BoxedWhereClause < 'a , DB > > , T : QuerySource , { type Output = BoxedDeleteStatement < 'a , DB , T , Ret > ; fn internal_into_boxed (self) -> Self :: Output { DeleteStatement { where_clause : self . where_clause . into () , returning : self . returning , from_clause : self . from_clause , } } }
};
}
