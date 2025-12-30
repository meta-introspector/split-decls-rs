// Generated macro for impl_351 (impl)
macro_rules! Depcrate_expression_coerceimpl_351 {
() => {
// Module: crate::expression::coerce
// Provides: {"impl_351"}
// Dependencies: {}
impl < T , ST , DB > QueryFragment < DB > for Coerce < T , ST > where T : QueryFragment < DB > , DB : Backend , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . expr . walk_ast (pass) } }
};
}
