// Generated macro for impl_388 (impl)
macro_rules! Depcrate_expression_existsimpl_388 {
() => {
// Module: crate::expression::exists
// Provides: {"impl_388"}
// Dependencies: {}
impl < T , DB > QueryFragment < DB > for Exists < T > where DB : Backend , Self : QueryFragment < DB , DB :: ExistsSyntax > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: ExistsSyntax > > :: walk_ast (self , pass) } }
};
}
