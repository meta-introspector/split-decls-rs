// Generated macro for impl_589 (impl)
macro_rules! Depcrate_types_from_sqlimpl_589 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_589"}
// Dependencies: {}
impl FromSql for Value { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { Ok (value . into ()) } }
};
}
