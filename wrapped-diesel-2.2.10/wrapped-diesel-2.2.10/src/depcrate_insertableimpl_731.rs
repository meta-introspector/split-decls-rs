// Generated macro for impl_731 (impl)
macro_rules! Depcrate_insertableimpl_731 {
() => {
// Module: crate::insertable
// Provides: {"impl_731"}
// Dependencies: {}
impl < Expr , DB > QueryFragment < DB , sql_dialect :: default_keyword_for_insert :: IsoSqlDefaultKeyword > for DefaultableColumnInsertValue < Expr > where DB : Backend + SqlDialect < InsertWithDefaultKeyword = sql_dialect :: default_keyword_for_insert :: IsoSqlDefaultKeyword > , Expr : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . unsafe_to_cache_prepared () ; if let Self :: Expression (ref inner) = * self { inner . walk_ast (out . reborrow ()) ? ; } else { out . push_sql ("DEFAULT") ; } Ok (()) } }
};
}
