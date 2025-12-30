// Generated macro for stmt_status (function)
macro_rules! Depcrate_raw_statementstmt_status {
() => {
// Module: crate::raw_statement
// Provides: {"stmt_status"}
// Dependencies: {}
# [inline] pub (crate) unsafe fn stmt_status (ptr : * mut ffi :: sqlite3_stmt , status : StatementStatus , reset : bool ,) -> i32 { assert ! (! ptr . is_null ()) ; ffi :: sqlite3_stmt_status (ptr , status as i32 , reset as i32) }
};
}
