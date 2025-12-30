// Generated macro for impl_4021 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4021 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4021"}
// Dependencies: {}
# [cfg (feature = "sqlite")] # [allow (clippy :: cast_possible_truncation)] impl FromSql < sql_types :: SmallInt , Sqlite > for i16 { fn from_sql (mut value : SqliteValue < '_ , '_ , '_ >) -> deserialize :: Result < Self > { Ok (value . read_integer () as i16) } }
};
}
