// Generated macro for impl_291 (impl)
macro_rules! Depcrate_expression_array_comparisonimpl_291 {
() => {
// Module: crate::expression::array_comparison
// Provides: {"impl_291"}
// Dependencies: {}
impl < T , U , DB > QueryFragment < DB > for NotIn < T , U > where DB : Backend , Self : QueryFragment < DB , DB :: ArrayComparison > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: ArrayComparison > > :: walk_ast (self , pass) } }
};
}
