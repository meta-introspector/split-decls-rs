// Generated macro for impl_510 (impl)
macro_rules! Depcrate_expression_operatorsimpl_510 {
() => {
// Module: crate::expression::operators
// Provides: {"impl_510"}
// Dependencies: {}
impl < L , R , DB > QueryFragment < DB > for Concat < L , R > where DB : Backend , Self : QueryFragment < DB , DB :: ConcatClause > , { fn walk_ast < 'b > (& 'b self , pass : crate :: query_builder :: AstPass < '_ , 'b , DB > ,) -> crate :: result :: QueryResult < () > { < Self as QueryFragment < DB , DB :: ConcatClause > > :: walk_ast (self , pass) } }
};
}
