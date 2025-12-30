// Generated macro for impl_3954 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_chronoimpl_3954 {
() => {
// Module: crate::sqlite::types::date_and_time::chrono
// Provides: {"impl_3954"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "chrono"))] impl FromSql < TimestamptzSqlite , Sqlite > for DateTime < Utc > { fn from_sql (mut value : < Sqlite as Backend > :: RawValue < '_ >) -> deserialize :: Result < Self > { if let Ok (dt) = value . parse_string (| text | { for format in DATETIME_FORMATS { if let Ok (dt) = DateTime :: parse_from_str (text , format) { return Ok (dt . with_timezone (& Utc)) ; } } Err (()) }) { return Ok (dt) ; } let naive_date_time = < NaiveDateTime as FromSql < TimestamptzSqlite , Sqlite > > :: from_sql (value) ? ; Ok (Utc . from_utc_datetime (& naive_date_time)) } }
};
}
