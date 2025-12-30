// Generated macro for impl_1531 (impl)
macro_rules! Depcrate_query_builderimpl_1531 {
() => {
// Module: crate::query_builder
// Provides: {"impl_1531"}
// Dependencies: {}
impl < T , DB > QueryFragment < DB > for Option < T > where DB : Backend , T : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { match * self { Some (ref c) => c . walk_ast (out) , None => Ok (()) , } } }
};
}
