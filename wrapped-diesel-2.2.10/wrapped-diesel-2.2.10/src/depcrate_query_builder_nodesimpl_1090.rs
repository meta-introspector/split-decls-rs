// Generated macro for impl_1090 (impl)
macro_rules! Depcrate_query_builder_nodesimpl_1090 {
() => {
// Module: crate::query_builder::nodes
// Provides: {"impl_1090"}
// Dependencies: {}
impl < T , U , DB , M > QueryFragment < DB > for InfixNode < T , U , M > where DB : Backend + DieselReserveSpecialization , T : QueryFragment < DB > , U : QueryFragment < DB > , M : MiddleFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . lhs . walk_ast (out . reborrow ()) ? ; self . middle . push_sql (out . reborrow ()) ; self . rhs . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
