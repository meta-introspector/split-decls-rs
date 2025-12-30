// Generated macro for Context (struct)
macro_rules! Depcrate_functionsContext {
() => {
// Module: crate::functions
// Provides: {"Context"}
// Dependencies: {}
# [doc = " Context is a wrapper for the SQLite function"] # [doc = " evaluation context."] pub struct Context < 'a > { ctx : * mut sqlite3_context , args : & 'a [* mut sqlite3_value] , }
};
}
