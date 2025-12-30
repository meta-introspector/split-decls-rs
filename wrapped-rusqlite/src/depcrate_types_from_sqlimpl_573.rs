// Generated macro for impl_573 (impl)
macro_rules! Depcrate_types_from_sqlimpl_573 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_573"}
// Dependencies: {}
impl FromSql for f32 { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { match value { ValueRef :: Integer (i) => Ok (i as Self) , ValueRef :: Real (f) => Ok (f as Self) , _ => Err (FromSqlError :: InvalidType) , } } }
};
}
