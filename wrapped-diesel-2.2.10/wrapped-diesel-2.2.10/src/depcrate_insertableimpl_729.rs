// Generated macro for impl_729 (impl)
macro_rules! Depcrate_insertableimpl_729 {
() => {
// Module: crate::insertable
// Provides: {"impl_729"}
// Dependencies: {}
impl < Col , Expr , DB > InsertValues < DB , Col :: Table > for ColumnInsertValue < Col , Expr > where DB : Backend , Col : Column , Expr : Expression < SqlType = Col :: SqlType > + AppearsOnTable < NoFromClause > , Self : QueryFragment < DB > , { fn column_names (& self , mut out : AstPass < '_ , '_ , DB >) -> QueryResult < () > { out . push_identifier (Col :: NAME) ? ; Ok (()) } }
};
}
