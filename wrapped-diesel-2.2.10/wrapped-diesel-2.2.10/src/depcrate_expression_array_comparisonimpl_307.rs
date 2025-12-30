// Generated macro for impl_307 (impl)
macro_rules! Depcrate_expression_array_comparisonimpl_307 {
() => {
// Module: crate::expression::array_comparison
// Provides: {"impl_307"}
// Dependencies: {}
impl < ST , I , DB > QueryFragment < DB > for Many < ST , I > where Self : QueryFragment < DB , DB :: ArrayComparison > , DB : Backend , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: ArrayComparison > > :: walk_ast (self , pass) } }
};
}
