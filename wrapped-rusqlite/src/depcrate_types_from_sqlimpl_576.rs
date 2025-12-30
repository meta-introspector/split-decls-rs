// Generated macro for impl_576 (impl)
macro_rules! Depcrate_types_from_sqlimpl_576 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_576"}
// Dependencies: {}
impl FromSql for String { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . map (ToString :: to_string) } }
};
}
