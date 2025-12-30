// Generated macro for impl_1746 (impl)
macro_rules! Depcrate_query_source_aliasing_aliasimpl_1746 {
() => {
// Module: crate::query_source::aliasing::alias
// Provides: {"impl_1746"}
// Dependencies: {}
impl < S , DB > QueryFragment < DB > for Alias < S > where S : AliasSource , DB : Backend , Self : QueryFragment < DB , DB :: AliasSyntax > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: AliasSyntax > > :: walk_ast (self , pass) } }
};
}
