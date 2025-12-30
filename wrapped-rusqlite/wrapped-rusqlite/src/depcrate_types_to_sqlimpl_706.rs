// Generated macro for impl_706 (impl)
macro_rules! Depcrate_types_to_sqlimpl_706 {
() => {
// Module: crate::types::to_sql
// Provides: {"impl_706"}
// Dependencies: {}
impl ToSql for str { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self)) } }
};
}
