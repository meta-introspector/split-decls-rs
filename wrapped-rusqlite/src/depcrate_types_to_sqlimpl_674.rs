// Generated macro for impl_674 (impl)
macro_rules! Depcrate_types_to_sqlimpl_674 {
() => {
// Module: crate::types::to_sql
// Provides: {"impl_674"}
// Dependencies: {}
impl < T : ToSql + ? Sized > ToSql for std :: sync :: Arc < T > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { self . as_ref () . to_sql () } }
};
}
