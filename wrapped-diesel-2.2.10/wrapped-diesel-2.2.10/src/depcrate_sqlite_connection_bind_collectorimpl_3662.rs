// Generated macro for impl_3662 (impl)
macro_rules! Depcrate_sqlite_connection_bind_collectorimpl_3662 {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"impl_3662"}
// Dependencies: {}
impl < 'a > BindCollector < 'a , Sqlite > for SqliteBindCollector < 'a > { type Buffer = SqliteBindValue < 'a > ; fn push_bound_value < T , U > (& mut self , bind : & 'a U , metadata_lookup : & mut ()) -> QueryResult < () > where Sqlite : crate :: sql_types :: HasSqlType < T > , U : crate :: serialize :: ToSql < T , Sqlite > + ? Sized , { let value = SqliteBindValue { inner : InternalSqliteBindValue :: Null , } ; let mut to_sql_output = Output :: new (value , metadata_lookup) ; let is_null = bind . to_sql (& mut to_sql_output) . map_err (crate :: result :: Error :: SerializationError) ? ; let bind = to_sql_output . into_inner () ; let metadata = Sqlite :: metadata (metadata_lookup) ; self . binds . push ((match is_null { IsNull :: No => bind . inner , IsNull :: Yes => InternalSqliteBindValue :: Null , } , metadata ,)) ; Ok (()) } fn push_null_value (& mut self , metadata : SqliteType) -> QueryResult < () > { self . binds . push ((InternalSqliteBindValue :: Null , metadata)) ; Ok (()) } }
};
}
