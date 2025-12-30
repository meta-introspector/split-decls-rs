// Generated macro for impl_708 (impl)
macro_rules! Depcrate_types_to_sqlimpl_708 {
() => {
// Module: crate::types::to_sql
// Provides: {"impl_708"}
// Dependencies: {}
impl < const N : usize > ToSql for [u8 ; N] { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (& self [..])) } }
};
}
