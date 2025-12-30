// Generated macro for impl_1528 (impl)
macro_rules! Depcrate_query_builderimpl_1528 {
() => {
// Module: crate::query_builder
// Provides: {"impl_1528"}
// Dependencies: {}
impl < T : ? Sized , DB > QueryFragment < DB > for Box < T > where DB : Backend , T : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { QueryFragment :: walk_ast (& * * self , pass) } }
};
}
