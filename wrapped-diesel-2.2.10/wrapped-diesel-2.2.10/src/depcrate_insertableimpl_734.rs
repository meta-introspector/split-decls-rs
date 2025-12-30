// Generated macro for impl_734 (impl)
macro_rules! Depcrate_insertableimpl_734 {
() => {
// Module: crate::insertable
// Provides: {"impl_734"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl < Col , Expr > QueryFragment < crate :: sqlite :: Sqlite , crate :: backend :: sql_dialect :: default_keyword_for_insert :: DoesNotSupportDefaultKeyword , > for DefaultableColumnInsertValue < ColumnInsertValue < Col , Expr > > where Expr : QueryFragment < crate :: sqlite :: Sqlite > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , crate :: sqlite :: Sqlite >) -> QueryResult < () > { if let Self :: Expression (ref inner) = * self { inner . walk_ast (out . reborrow ()) ? ; } Ok (()) } }
};
}
