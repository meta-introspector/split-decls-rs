// Generated macro for debug (function)
macro_rules! Depcrate_query_builder_debug_querydebug {
() => {
// Module: crate::query_builder::debug_query
// Provides: {"debug"}
// Dependencies: {}
fn debug < DB > (query : & dyn QueryFragment < DB > , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result where DB : Backend + Default , DB :: QueryBuilder : Default , { let debug_binds = DebugBinds :: < DB > :: new (query) ; let query = serialize_query (query) ? ; f . debug_struct ("Query") . field ("sql" , & query) . field ("binds" , & debug_binds) . finish () }
};
}
