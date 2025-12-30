// Generated macro for impl_83 (impl)
macro_rules! Depcrate_sqliteimpl_83 {
() => {
// Module: crate::sqlite
// Provides: {"impl_83"}
// Dependencies: {}
impl FromSql < sql_types :: Date , Sqlite > for Date { fn from_sql (value : SqliteValue < '_ , '_ , '_ >) -> deserialize :: Result < Date > { let text : String = FromSql :: < sql_types :: Date , Sqlite > :: from_sql (value) ? ; let date = PARSER . parse_date (text) ? ; Ok (date . to_diesel ()) } }
};
}
