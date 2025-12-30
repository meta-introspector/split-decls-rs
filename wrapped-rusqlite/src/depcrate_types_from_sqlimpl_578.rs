// Generated macro for impl_578 (impl)
macro_rules! Depcrate_types_from_sqlimpl_578 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_578"}
// Dependencies: {}
impl FromSql for std :: rc :: Rc < str > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . map (Into :: into) } }
};
}
