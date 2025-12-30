// Generated macro for impl_577 (impl)
macro_rules! Depcrate_types_from_sqlimpl_577 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_577"}
// Dependencies: {}
impl FromSql for Box < str > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . map (Into :: into) } }
};
}
