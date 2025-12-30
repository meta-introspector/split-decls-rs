// Generated macro for register (function)
macro_rules! Depcrate_sqlite_connection_functionsregister {
() => {
// Module: crate::sqlite::connection::functions
// Provides: {"register"}
// Dependencies: {}
pub (super) fn register < ArgsSqlType , RetSqlType , Args , Ret , F > (conn : & RawConnection , fn_name : & str , deterministic : bool , mut f : F ,) -> QueryResult < () > where F : FnMut (& RawConnection , Args) -> Ret + std :: panic :: UnwindSafe + Send + 'static , Args : FromSqlRow < ArgsSqlType , Sqlite > + StaticallySizedRow < ArgsSqlType , Sqlite > , Ret : ToSql < RetSqlType , Sqlite > , Sqlite : HasSqlType < RetSqlType > , { let fields_needed = Args :: FIELD_COUNT ; if fields_needed > 127 { return Err (Error :: DatabaseError (DatabaseErrorKind :: UnableToSendCommand , Box :: new ("SQLite functions cannot take more than 127 parameters" . to_string ()) ,)) ; } conn . register_sql_function (fn_name , fields_needed , deterministic , move | conn , args | { let args = build_sql_function_args :: < ArgsSqlType , Args > (args) ? ; Ok (f (conn , args)) }) ? ; Ok (()) }
};
}
