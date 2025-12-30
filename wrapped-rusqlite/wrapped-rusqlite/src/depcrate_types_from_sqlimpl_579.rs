// Generated macro for impl_579 (impl)
macro_rules! Depcrate_types_from_sqlimpl_579 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_579"}
// Dependencies: {}
impl FromSql for std :: sync :: Arc < str > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . map (Into :: into) } }
};
}
