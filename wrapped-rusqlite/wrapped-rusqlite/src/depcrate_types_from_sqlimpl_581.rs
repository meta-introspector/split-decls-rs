// Generated macro for impl_581 (impl)
macro_rules! Depcrate_types_from_sqlimpl_581 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_581"}
// Dependencies: {}
impl FromSql for Box < [u8] > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_blob () . map (Box :: < [u8] > :: from) } }
};
}
