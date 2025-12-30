// Generated macro for impl_1948 (impl)
macro_rules! Depcrate_r2d2impl_1948 {
() => {
// Module: crate::r2d2
// Provides: {"impl_1948"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for CheckConnectionQuery where DB : Backend , { fn walk_ast < 'b > (& 'b self , mut pass : crate :: query_builder :: AstPass < '_ , 'b , DB > ,) -> QueryResult < () > { pass . push_sql ("SELECT 1") ; Ok (()) } }
};
}
