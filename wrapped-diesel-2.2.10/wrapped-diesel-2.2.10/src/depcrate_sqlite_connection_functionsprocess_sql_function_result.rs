// Generated macro for process_sql_function_result (function)
macro_rules! Depcrate_sqlite_connection_functionsprocess_sql_function_result {
() => {
// Module: crate::sqlite::connection::functions
// Provides: {"process_sql_function_result"}
// Dependencies: {}
# [allow (clippy :: let_unit_value)] pub (super) fn process_sql_function_result < RetSqlType , Ret > (result : & '_ Ret ,) -> QueryResult < InternalSqliteBindValue < '_ > > where Ret : ToSql < RetSqlType , Sqlite > , Sqlite : HasSqlType < RetSqlType > , { let mut metadata_lookup = () ; let value = SqliteBindValue { inner : InternalSqliteBindValue :: Null , } ; let mut buf = Output :: new (value , & mut metadata_lookup) ; let is_null = result . to_sql (& mut buf) . map_err (Error :: SerializationError) ? ; if let IsNull :: Yes = is_null { Ok (InternalSqliteBindValue :: Null) } else { Ok (buf . into_inner () . inner) } }
};
}
