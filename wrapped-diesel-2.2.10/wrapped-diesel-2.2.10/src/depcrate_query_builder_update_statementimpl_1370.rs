// Generated macro for impl_1370 (impl)
macro_rules! Depcrate_query_builder_update_statementimpl_1370 {
() => {
// Module: crate::query_builder::update_statement
// Provides: {"impl_1370"}
// Dependencies: {}
impl < T , U , V , Ret , Predicate > FilterDsl < Predicate > for UpdateStatement < T , U , V , Ret > where T : QuerySource , U : WhereAnd < Predicate > , Predicate : AppearsOnTable < T > , { type Output = UpdateStatement < T , U :: Output , V , Ret > ; fn filter (self , predicate : Predicate) -> Self :: Output { UpdateStatement { from_clause : self . from_clause , where_clause : self . where_clause . and (predicate) , values : self . values , returning : self . returning , } } }
};
}
