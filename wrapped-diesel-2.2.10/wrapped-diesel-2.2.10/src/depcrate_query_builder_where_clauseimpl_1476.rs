// Generated macro for impl_1476 (impl)
macro_rules! Depcrate_query_builder_where_clauseimpl_1476 {
() => {
// Module: crate::query_builder::where_clause
// Provides: {"impl_1476"}
// Dependencies: {}
impl < DB , Expr > QueryFragment < DB > for WhereClause < Expr > where DB : Backend + DieselReserveSpecialization , Expr : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql (" WHERE ") ; self . 0 . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
