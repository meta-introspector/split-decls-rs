// Generated macro for impl_733 (impl)
macro_rules! Depcrate_insertableimpl_733 {
() => {
// Module: crate::insertable
// Provides: {"impl_733"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl < Col , Expr > InsertValues < crate :: sqlite :: Sqlite , Col :: Table > for DefaultableColumnInsertValue < ColumnInsertValue < Col , Expr > > where Col : Column , Expr : Expression < SqlType = Col :: SqlType > + AppearsOnTable < NoFromClause > , Self : QueryFragment < crate :: sqlite :: Sqlite > , { fn column_names (& self , mut out : AstPass < '_ , '_ , crate :: sqlite :: Sqlite >) -> QueryResult < () > { if let Self :: Expression (..) = * self { out . push_identifier (Col :: NAME) ? ; } Ok (()) } }
};
}
