// Generated macro for impl_865 (impl)
macro_rules! Depcrate_query_builder_delete_statementimpl_865 {
() => {
// Module: crate::query_builder::delete_statement
// Provides: {"impl_865"}
// Dependencies: {}
impl < T , U , Ret , DB > QueryFragment < DB > for DeleteStatement < T , U , Ret > where DB : Backend + DieselReserveSpecialization , T : Table , FromClause < T > : QueryFragment < DB > , U : QueryFragment < DB > , Ret : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("DELETE") ; self . from_clause . walk_ast (out . reborrow ()) ? ; self . where_clause . walk_ast (out . reborrow ()) ? ; self . returning . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
