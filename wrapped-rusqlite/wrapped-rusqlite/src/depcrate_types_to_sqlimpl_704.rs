// Generated macro for impl_704 (impl)
macro_rules! Depcrate_types_to_sqlimpl_704 {
() => {
// Module: crate::types::to_sql
// Provides: {"impl_704"}
// Dependencies: {}
impl < T : ? Sized > ToSql for & '_ T where T : ToSql , { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { (* self) . to_sql () } }
};
}
