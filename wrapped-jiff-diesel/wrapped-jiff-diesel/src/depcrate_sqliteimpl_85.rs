// Generated macro for impl_85 (impl)
macro_rules! Depcrate_sqliteimpl_85 {
() => {
// Module: crate::sqlite
// Provides: {"impl_85"}
// Dependencies: {}
impl FromSql < sql_types :: Time , Sqlite > for Time { fn from_sql (value : SqliteValue < '_ , '_ , '_ >) -> deserialize :: Result < Time > { let text : String = FromSql :: < sql_types :: Time , Sqlite > :: from_sql (value) ? ; let time = PARSER . parse_time (text) ? ; Ok (time . to_diesel ()) } }
};
}
