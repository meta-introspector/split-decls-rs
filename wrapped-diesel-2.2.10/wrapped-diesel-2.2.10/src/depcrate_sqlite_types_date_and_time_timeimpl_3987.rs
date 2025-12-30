// Generated macro for impl_3987 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_timeimpl_3987 {
() => {
// Module: crate::sqlite::types::date_and_time::time
// Provides: {"impl_3987"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "time"))] impl FromSql < TimestamptzSqlite , Sqlite > for OffsetDateTime { fn from_sql (mut value : < Sqlite as Backend > :: RawValue < '_ >) -> deserialize :: Result < Self > { if let Ok (dt) = value . parse_string (| text | { for format in DATETIME_FORMATS { if let Ok (dt) = OffsetDateTime :: parse (text , format) { return Ok (dt) ; } } Err (()) }) { return Ok (dt) ; } let primitive_date_time = < PrimitiveDateTime as FromSql < TimestamptzSqlite , Sqlite > > :: from_sql (value) ? ; Ok (primitive_date_time . assume_utc ()) } }
};
}
