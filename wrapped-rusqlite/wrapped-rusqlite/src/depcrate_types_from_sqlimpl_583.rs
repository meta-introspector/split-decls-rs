// Generated macro for impl_583 (impl)
macro_rules! Depcrate_types_from_sqlimpl_583 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_583"}
// Dependencies: {}
impl FromSql for std :: sync :: Arc < [u8] > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_blob () . map (std :: sync :: Arc :: < [u8] > :: from) } }
};
}
