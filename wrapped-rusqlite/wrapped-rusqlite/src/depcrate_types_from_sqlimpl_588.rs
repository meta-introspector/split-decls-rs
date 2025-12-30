// Generated macro for impl_588 (impl)
macro_rules! Depcrate_types_from_sqlimpl_588 {
() => {
// Module: crate::types::from_sql
// Provides: {"impl_588"}
// Dependencies: {}
impl < T : ? Sized > FromSql for Cow < '_ , T > where T : ToOwned , T :: Owned : FromSql , { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { < T :: Owned > :: column_result (value) . map (Cow :: Owned) } }
};
}
