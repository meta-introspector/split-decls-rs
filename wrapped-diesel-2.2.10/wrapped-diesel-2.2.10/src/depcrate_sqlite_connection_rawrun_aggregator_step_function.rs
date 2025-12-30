// Generated macro for run_aggregator_step_function (function)
macro_rules! Depcrate_sqlite_connection_rawrun_aggregator_step_function {
() => {
// Module: crate::sqlite::connection::raw
// Provides: {"run_aggregator_step_function"}
// Dependencies: {}
# [allow (warnings)] extern "C" fn run_aggregator_step_function < ArgsSqlType , RetSqlType , Args , Ret , A > (ctx : * mut ffi :: sqlite3_context , num_args : libc :: c_int , value_ptr : * mut * mut ffi :: sqlite3_value ,) where A : SqliteAggregateFunction < Args , Output = Ret > + 'static + Send + std :: panic :: UnwindSafe , Args : FromSqlRow < ArgsSqlType , Sqlite > , Ret : ToSql < RetSqlType , Sqlite > , Sqlite : HasSqlType < RetSqlType > , { let result = std :: panic :: catch_unwind (move | | { let args = unsafe { slice :: from_raw_parts_mut (value_ptr , num_args as _) } ; run_aggregator_step :: < A , Args , ArgsSqlType > (ctx , args) }) . unwrap_or_else (| e | { Err (SqliteCallbackError :: Panic (format ! ("{}::step() panicked" , std :: any :: type_name ::< A > ()))) }) ; match result { Ok (()) => { } Err (e) => e . emit (ctx) , } }
};
}
