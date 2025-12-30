// Generated macro for impl_707 (impl)
macro_rules! Depcrate_types_to_sqlimpl_707 {
() => {
// Module: crate::types::to_sql
// Provides: {"impl_707"}
// Dependencies: {}
impl ToSql for Vec < u8 > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self . as_slice ())) } }
};
}
