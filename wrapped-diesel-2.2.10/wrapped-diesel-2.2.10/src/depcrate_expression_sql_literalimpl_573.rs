// Generated macro for impl_573 (impl)
macro_rules! Depcrate_expression_sql_literalimpl_573 {
() => {
// Module: crate::expression::sql_literal
// Provides: {"impl_573"}
// Dependencies: {}
impl < Query , Value , DB > QueryFragment < DB > for UncheckedBind < Query , Value > where DB : Backend , Query : QueryFragment < DB > , Value : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . query . walk_ast (out . reborrow ()) ? ; self . value . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
