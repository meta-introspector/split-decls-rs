// Generated macro for expanded_sql (function)
macro_rules! Depcrate_raw_statementexpanded_sql {
() => {
// Module: crate::raw_statement
// Provides: {"expanded_sql"}
// Dependencies: {}
# [inline] pub (crate) unsafe fn expanded_sql (ptr : * mut ffi :: sqlite3_stmt) -> Option < SqliteMallocString > { SqliteMallocString :: from_raw (ffi :: sqlite3_expanded_sql (ptr)) }
};
}
