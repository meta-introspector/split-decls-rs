// Generated macro for impl_582 (impl)
macro_rules! Depcrate_types_from_sqlimpl_582 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_582"}
// Dependencies: {}
impl FromSql for std :: rc :: Rc < [u8] > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_blob () . map (std :: rc :: Rc :: < [u8] > :: from) } }
};
}
