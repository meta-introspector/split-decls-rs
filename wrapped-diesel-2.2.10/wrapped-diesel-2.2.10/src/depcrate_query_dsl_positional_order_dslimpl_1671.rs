// Generated macro for impl_1671 (impl)
macro_rules! Depcrate_query_dsl_positional_order_dslimpl_1671 {
() => {
// Module: crate::query_dsl::positional_order_dsl
// Provides: {"impl_1671"}
// Dependencies: {}
impl < Source , Expr , DB > QueryFragment < DB > for PositionalOrderClause < Source , Expr > where DB : Backend + DieselReserveSpecialization , Source : QueryFragment < DB > , Expr : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . source . walk_ast (pass . reborrow ()) ? ; pass . push_sql (" ORDER BY ") ; self . expr . walk_ast (pass) } }
};
}
