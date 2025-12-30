// Generated macro for impl_732 (impl)
macro_rules! Depcrate_insertableimpl_732 {
() => {
// Module: crate::insertable
// Provides: {"impl_732"}
// Dependencies: {}
impl < Col , Expr , DB > QueryFragment < DB > for ColumnInsertValue < Col , Expr > where DB : Backend + DieselReserveSpecialization , Expr : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . expr . walk_ast (pass) } }
};
}
