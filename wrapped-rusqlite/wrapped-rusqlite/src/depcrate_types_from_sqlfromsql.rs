// Generated macro for FromSql (trait)
macro_rules! Depcrate_types_from_sqlFromSql {
() => {
// Module: crate::types::from_sql
// Provides: {"FromSql"}
// Dependencies: {}
# [doc = " A trait for types that can be created from a SQLite value."] pub trait FromSql : Sized { # [doc = " Converts SQLite value into Rust value."] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > ; }
};
}
