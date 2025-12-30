// Generated macro for impl_671 (impl)
macro_rules! Depcrate_types_to_sqlimpl_671 {
() => {
// Module: crate::types::to_sql
// Provides: {"impl_671"}
// Dependencies: {}
impl < T : ToSql + ToOwned + ? Sized > ToSql for Cow < '_ , T > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { self . as_ref () . to_sql () } }
};
}
