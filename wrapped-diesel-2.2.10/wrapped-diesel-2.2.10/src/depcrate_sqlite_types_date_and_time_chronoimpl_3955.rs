// Generated macro for impl_3955 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_chronoimpl_3955 {
() => {
// Module: crate::sqlite::types::date_and_time::chrono
// Provides: {"impl_3955"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "chrono"))] impl FromSql < TimestamptzSqlite , Sqlite > for DateTime < Local > { fn from_sql (mut value : < Sqlite as Backend > :: RawValue < '_ >) -> deserialize :: Result < Self > { if let Ok (dt) = value . parse_string (| text | { for format in DATETIME_FORMATS { if let Ok (dt) = DateTime :: parse_from_str (text , format) { return Ok (dt . with_timezone (& Local)) ; } } Err (()) }) { return Ok (dt) ; } let naive_date_time = < NaiveDateTime as FromSql < TimestamptzSqlite , Sqlite > > :: from_sql (value) ? ; Ok (Local :: from_utc_datetime (& Local , & naive_date_time)) } }
};
}
