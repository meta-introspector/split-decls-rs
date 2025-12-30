// Generated macro for impl_1083 (impl)
macro_rules! Depcrate_query_builder_nodesimpl_1083 {
() => {
// Module: crate::query_builder::nodes
// Provides: {"impl_1083"}
// Dependencies: {}
impl < T , DB > QueryFragment < DB > for StaticQueryFragmentInstance < T > where DB : Backend + DieselReserveSpecialization , T : StaticQueryFragment , T :: Component : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { T :: STATIC_COMPONENT . walk_ast (pass) } }
};
}
