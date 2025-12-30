// Generated macro for impl_705 (impl)
macro_rules! Depcrate_types_to_sqlimpl_705 {
() => {
// Module: crate::types::to_sql
// Provides: {"impl_705"}
// Dependencies: {}
impl ToSql for String { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self . as_str ())) } }
};
}
