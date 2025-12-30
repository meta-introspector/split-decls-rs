// Generated macro for impl_580 (impl)
macro_rules! Depcrate_types_from_sqlimpl_580 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_580"}
// Dependencies: {}
impl FromSql for Vec < u8 > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_blob () . map (< [u8] > :: to_vec) } }
};
}
