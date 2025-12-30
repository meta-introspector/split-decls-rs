// Generated macro for ToSql (trait)
macro_rules! Depcrate_types_to_sqlToSql {
() => {
// Module: crate::types::to_sql
// Provides: {"ToSql"}
// Dependencies: {}
# [doc = " A trait for types that can be converted into SQLite values. Returns"] # [doc = " [`crate::Error::ToSqlConversionFailure`] if the conversion fails."] pub trait ToSql { # [doc = " Converts Rust value to SQLite value"] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > ; }
};
}
