// Generated macro for impl_711 (impl)
macro_rules! Depcrate_types_to_sqlimpl_711 {
() => {
// Module: crate::types::to_sql
// Provides: {"impl_711"}
// Dependencies: {}
impl < T : ToSql > ToSql for Option < T > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { match * self { None => Ok (ToSqlOutput :: from (Null)) , Some (ref t) => t . to_sql () , } } }
};
}
