// Generated macro for impl_730 (impl)
macro_rules! Depcrate_insertableimpl_730 {
() => {
// Module: crate::insertable
// Provides: {"impl_730"}
// Dependencies: {}
impl < Expr , DB > QueryFragment < DB > for DefaultableColumnInsertValue < Expr > where DB : Backend , Self : QueryFragment < DB , DB :: InsertWithDefaultKeyword > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: InsertWithDefaultKeyword > > :: walk_ast (self , pass) } }
};
}
