// Generated macro for display (function)
macro_rules! Depcrate_query_builder_debug_querydisplay {
() => {
// Module: crate::query_builder::debug_query
// Provides: {"display"}
// Dependencies: {}
fn display < DB > (query : & dyn QueryFragment < DB > , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result where DB : Backend + Default , DB :: QueryBuilder : Default , { let debug_binds = DebugBinds :: < DB > :: new (query) ; let query = serialize_query (query) ? ; write ! (f , "{} -- binds: {:?}" , query , debug_binds) }
};
}
