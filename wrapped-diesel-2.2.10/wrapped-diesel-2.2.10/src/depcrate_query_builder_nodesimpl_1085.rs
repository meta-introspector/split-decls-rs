// Generated macro for impl_1085 (impl)
macro_rules! Depcrate_query_builder_nodesimpl_1085 {
() => {
// Module: crate::query_builder::nodes
// Provides: {"impl_1085"}
// Dependencies: {}
impl < DB : Backend > QueryFragment < DB > for Identifier < '_ > { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_identifier (self . 0) } }
};
}
