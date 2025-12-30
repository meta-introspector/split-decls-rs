// Generated macro for run_aggregator_step (function)
macro_rules! Depcrate_sqlite_connection_rawrun_aggregator_step {
() => {
// Module: crate::sqlite::connection::raw
// Provides: {"run_aggregator_step"}
// Dependencies: {}
fn run_aggregator_step < A , Args , ArgsSqlType > (ctx : * mut ffi :: sqlite3_context , args : & mut [* mut ffi :: sqlite3_value] ,) -> Result < () , SqliteCallbackError > where A : SqliteAggregateFunction < Args > , Args : FromSqlRow < ArgsSqlType , Sqlite > , { static NULL_AG_CTX_ERR : & str = "An unknown error occurred. sqlite3_aggregate_context returned a null pointer. This should never happen." ; static NULL_CTX_ERR : & str = "We've written the aggregator to the aggregate context, but it could not be retrieved." ; let n_bytes : i32 = std :: mem :: size_of :: < OptionalAggregator < A > > () . try_into () . expect ("Aggregate context should be larger than 2^32") ; let aggregate_context = unsafe { ffi :: sqlite3_aggregate_context (ctx , n_bytes) } ; let aggregate_context = NonNull :: new (aggregate_context as * mut OptionalAggregator < A >) ; let aggregator = unsafe { match aggregate_context . map (| a | & mut * a . as_ptr ()) { Some (& mut OptionalAggregator :: Some (ref mut agg)) => agg , Some (a_ptr @ & mut OptionalAggregator :: None) => { ptr :: write_unaligned (a_ptr as * mut _ , OptionalAggregator :: Some (A :: default ())) ; if let OptionalAggregator :: Some (ref mut agg) = a_ptr { agg } else { return Err (SqliteCallbackError :: Abort (NULL_CTX_ERR)) ; } } None => { return Err (SqliteCallbackError :: Abort (NULL_AG_CTX_ERR)) ; } } } ; let args = build_sql_function_args :: < ArgsSqlType , Args > (args) ? ; aggregator . step (args) ; Ok (()) }
};
}
