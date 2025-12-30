// Generated macro for sql_result (function)
macro_rules! Depcrate_functionssql_result {
() => {
// Module: crate::functions
// Provides: {"sql_result"}
// Dependencies: {}
unsafe fn sql_result < T : SqlFnOutput > (ctx : * mut sqlite3_context , args : & [* mut sqlite3_value] , r : Result < T > ,) { let t = r . as_ref () . map (SqlFnOutput :: to_sql) ; match t { Ok (Ok ((ref value , sub_type))) => { set_result (ctx , args , value) ; if let Some (sub_type) = sub_type { ffi :: sqlite3_result_subtype (ctx , sub_type) ; } } Ok (Err (err)) => report_error (ctx , & err) , Err (err) => report_error (ctx , err) , } ; }
};
}
