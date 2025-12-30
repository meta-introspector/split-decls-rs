// Generated macro for impl_320 (impl)
macro_rules! Depcrate_expression_assume_not_nullimpl_320 {
() => {
// Module: crate::expression::assume_not_null
// Provides: {"impl_320"}
// Dependencies: {}
impl < T , DB > QueryFragment < DB > for AssumeNotNull < T > where DB : Backend , T : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . 0 . walk_ast (pass) } }
};
}
