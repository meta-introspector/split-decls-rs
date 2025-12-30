// Generated macro for impl_4025 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4025 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4025"}
// Dependencies: {}
# [cfg (feature = "sqlite")] # [allow (clippy :: cast_possible_truncation)] impl FromSql < sql_types :: Float , Sqlite > for f32 { fn from_sql (mut value : SqliteValue < '_ , '_ , '_ >) -> deserialize :: Result < Self > { Ok (value . read_double () as f32) } }
};
}
