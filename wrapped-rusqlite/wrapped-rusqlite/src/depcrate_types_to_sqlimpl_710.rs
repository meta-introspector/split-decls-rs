// Generated macro for impl_710 (impl)
macro_rules! Depcrate_types_to_sqlimpl_710 {
() => {
// Module: crate::types::to_sql
// Provides: {"impl_710"}
// Dependencies: {}
impl ToSql for Value { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self)) } }
};
}
