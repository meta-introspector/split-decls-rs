// Generated macro for impl_862 (impl)
macro_rules! Depcrate_query_builder_delete_statementimpl_862 {
() => {
// Module: crate::query_builder::delete_statement
// Provides: {"impl_862"}
// Dependencies: {}
impl < T , U , Ret , Predicate > FilterDsl < Predicate > for DeleteStatement < T , U , Ret > where U : WhereAnd < Predicate > , Predicate : AppearsOnTable < T > , T : QuerySource , { type Output = DeleteStatement < T , U :: Output , Ret > ; fn filter (self , predicate : Predicate) -> Self :: Output { DeleteStatement { from_clause : self . from_clause , where_clause : self . where_clause . and (predicate) , returning : self . returning , } } }
};
}
