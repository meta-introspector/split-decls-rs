// Generated macro for impl_587 (impl)
macro_rules! Depcrate_types_from_sqlimpl_587 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_587"}
// Dependencies: {}
impl < T : FromSql > FromSql for Option < T > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { match value { ValueRef :: Null => Ok (None) , _ => FromSql :: column_result (value) . map (Some) , } } }
};
}
