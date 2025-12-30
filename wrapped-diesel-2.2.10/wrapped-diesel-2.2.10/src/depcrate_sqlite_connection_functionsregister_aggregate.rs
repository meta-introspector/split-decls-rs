// Generated macro for register_aggregate (function)
macro_rules! Depcrate_sqlite_connection_functionsregister_aggregate {
() => {
// Module: crate::sqlite::connection::functions
// Provides: {"register_aggregate"}
// Dependencies: {}
pub (super) fn register_aggregate < ArgsSqlType , RetSqlType , Args , Ret , A > (conn : & RawConnection , fn_name : & str ,) -> QueryResult < () > where A : SqliteAggregateFunction < Args , Output = Ret > + 'static + Send + std :: panic :: UnwindSafe , Args : FromSqlRow < ArgsSqlType , Sqlite > + StaticallySizedRow < ArgsSqlType , Sqlite > , Ret : ToSql < RetSqlType , Sqlite > , Sqlite : HasSqlType < RetSqlType > , { let fields_needed = Args :: FIELD_COUNT ; if fields_needed > 127 { return Err (Error :: DatabaseError (DatabaseErrorKind :: UnableToSendCommand , Box :: new ("SQLite functions cannot take more than 127 parameters" . to_string ()) ,)) ; } conn . register_aggregate_function :: < ArgsSqlType , RetSqlType , Args , Ret , A > (fn_name , fields_needed ,) ? ; Ok (()) }
};
}
