// Generated macro for impl_857 (impl)
macro_rules! Depcrate_query_builder_delete_statementimpl_857 {
() => {
// Module: crate::query_builder::delete_statement
// Provides: {"impl_857"}
// Dependencies: {}
impl < T , U , Ret > Clone for DeleteStatement < T , U , Ret > where T : QuerySource , FromClause < T > : Clone , U : Clone , Ret : Clone , { fn clone (& self) -> Self { Self { from_clause : self . from_clause . clone () , where_clause : self . where_clause . clone () , returning : self . returning . clone () , } } }
};
}
