// Generated macro for impl_3919 (impl)
macro_rules! Depcrate_sqlite_query_builder_returningimpl_3919 {
() => {
// Module: crate::sqlite::query_builder::returning
// Provides: {"impl_3919"}
// Dependencies: {}
impl < Expr , DB > QueryFragment < DB , SqliteReturningClause > for ReturningClause < Expr > where DB : Backend < ReturningClause = SqliteReturningClause > , Expr : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . skip_from (true) ; out . push_sql (" RETURNING ") ; self . 0 . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
