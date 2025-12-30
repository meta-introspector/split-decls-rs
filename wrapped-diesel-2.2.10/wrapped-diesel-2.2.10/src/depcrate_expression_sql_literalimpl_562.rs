// Generated macro for impl_562 (impl)
macro_rules! Depcrate_expression_sql_literalimpl_562 {
() => {
// Module: crate::expression::sql_literal
// Provides: {"impl_562"}
// Dependencies: {}
impl < ST , T , DB > QueryFragment < DB > for SqlLiteral < ST , T > where DB : Backend , T : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . unsafe_to_cache_prepared () ; self . inner . walk_ast (out . reborrow ()) ? ; out . push_sql (& self . sql) ; Ok (()) } }
};
}
