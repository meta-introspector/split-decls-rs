// Generated macro for impl_81 (impl)
macro_rules! Depcrate_sqliteimpl_81 {
() => {
// Module: crate::sqlite
// Provides: {"impl_81"}
// Dependencies: {}
impl FromSql < sql_types :: Timestamp , Sqlite > for DateTime { fn from_sql (value : SqliteValue < '_ , '_ , '_ > ,) -> deserialize :: Result < DateTime > { let text : String = FromSql :: < sql_types :: Timestamp , Sqlite > :: from_sql (value) ? ; let dt = PARSER . parse_datetime (text) ? ; Ok (dt . to_diesel ()) } }
};
}
