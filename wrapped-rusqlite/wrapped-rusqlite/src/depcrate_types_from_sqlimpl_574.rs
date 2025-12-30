// Generated macro for impl_574 (impl)
macro_rules! Depcrate_types_from_sqlimpl_574 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_574"}
// Dependencies: {}
impl FromSql for f64 { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { match value { ValueRef :: Integer (i) => Ok (i as Self) , ValueRef :: Real (f) => Ok (f) , _ => Err (FromSqlError :: InvalidType) , } } }
};
}
