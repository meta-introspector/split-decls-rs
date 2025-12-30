// Generated macro for impl_79 (impl)
macro_rules! Depcrate_sqliteimpl_79 {
() => {
// Module: crate::sqlite
// Provides: {"impl_79"}
// Dependencies: {}
impl FromSql < sql_types :: TimestamptzSqlite , Sqlite > for Timestamp { fn from_sql (value : SqliteValue < '_ , '_ , '_ > ,) -> deserialize :: Result < Timestamp > { let text : String = FromSql :: < sql_types :: Timestamp , Sqlite > :: from_sql (value) ? ; if text . contains (':') { let date = PARSER . parse_timestamp (text) ? ; return Ok (date . to_diesel ()) ; } let julian_days = text . parse :: < f64 > () ? ; julian_days_to_timestamp (julian_days) . map (jiff :: Timestamp :: to_diesel) } }
};
}
