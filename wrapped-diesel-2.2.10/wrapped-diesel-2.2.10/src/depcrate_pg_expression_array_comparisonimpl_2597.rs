// Generated macro for impl_2597 (impl)
macro_rules! Depcrate_pg_expression_array_comparisonimpl_2597 {
() => {
// Module: crate::pg::expression::array_comparison
// Provides: {"impl_2597"}
// Dependencies: {}
impl < Expr > QueryFragment < Pg > for All < Expr > where Expr : QueryFragment < Pg > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { out . push_sql ("ALL(") ; self . expr . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; Ok (()) } }
};
}
