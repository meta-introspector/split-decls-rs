// Generated macro for build_sql_function_args (function)
macro_rules! Depcrate_sqlite_connection_functionsbuild_sql_function_args {
() => {
// Module: crate::sqlite::connection::functions
// Provides: {"build_sql_function_args"}
// Dependencies: {}
pub (super) fn build_sql_function_args < ArgsSqlType , Args > (args : & mut [* mut ffi :: sqlite3_value] ,) -> Result < Args , Error > where Args : FromSqlRow < ArgsSqlType , Sqlite > , { let row = FunctionRow :: new (args) ; Args :: build_from_row (& row) . map_err (Error :: DeserializationError) }
};
}
