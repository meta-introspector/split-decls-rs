// Generated macro for impl_572 (impl)
macro_rules! Depcrate_types_from_sqlimpl_572 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_572"}
// Dependencies: {}
impl FromSql for i64 { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_i64 () } }
};
}
