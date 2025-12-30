// Generated macro for impl_575 (impl)
macro_rules! Depcrate_types_from_sqlimpl_575 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_575"}
// Dependencies: {}
impl FromSql for bool { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { i64 :: column_result (value) . map (| i | i != 0) } }
};
}
