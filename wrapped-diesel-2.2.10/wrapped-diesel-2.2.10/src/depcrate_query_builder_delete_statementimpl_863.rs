// Generated macro for impl_863 (impl)
macro_rules! Depcrate_query_builder_delete_statementimpl_863 {
() => {
// Module: crate::query_builder::delete_statement
// Provides: {"impl_863"}
// Dependencies: {}
impl < T , U , Ret , Predicate > OrFilterDsl < Predicate > for DeleteStatement < T , U , Ret > where T : QuerySource , U : WhereOr < Predicate > , Predicate : AppearsOnTable < T > , { type Output = DeleteStatement < T , U :: Output , Ret > ; fn or_filter (self , predicate : Predicate) -> Self :: Output { DeleteStatement { from_clause : self . from_clause , where_clause : self . where_clause . or (predicate) , returning : self . returning , } } }
};
}
