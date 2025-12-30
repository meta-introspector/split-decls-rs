// Generated macro for impl_1371 (impl)
macro_rules! Depcrate_query_builder_update_statementimpl_1371 {
() => {
// Module: crate::query_builder::update_statement
// Provides: {"impl_1371"}
// Dependencies: {}
impl < 'a , T , U , V , Ret , DB > BoxedDsl < 'a , DB > for UpdateStatement < T , U , V , Ret > where T : QuerySource , U : Into < BoxedWhereClause < 'a , DB > > , { type Output = BoxedUpdateStatement < 'a , DB , T , V , Ret > ; fn internal_into_boxed (self) -> Self :: Output { UpdateStatement { from_clause : self . from_clause , where_clause : self . where_clause . into () , values : self . values , returning : self . returning , } } }
};
}
