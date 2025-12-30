// Generated macro for impl_728 (impl)
macro_rules! Depcrate_insertableimpl_728 {
() => {
// Module: crate::insertable
// Provides: {"impl_728"}
// Dependencies: {}
impl < Col , Expr , DB > InsertValues < DB , Col :: Table > for DefaultableColumnInsertValue < ColumnInsertValue < Col , Expr > > where DB : Backend + SqlDialect < InsertWithDefaultKeyword = sql_dialect :: default_keyword_for_insert :: IsoSqlDefaultKeyword > , Col : Column , Expr : Expression < SqlType = Col :: SqlType > + AppearsOnTable < NoFromClause > , Self : QueryFragment < DB > , { fn column_names (& self , mut out : AstPass < '_ , '_ , DB >) -> QueryResult < () > { out . push_identifier (Col :: NAME) ? ; Ok (()) } }
};
}
