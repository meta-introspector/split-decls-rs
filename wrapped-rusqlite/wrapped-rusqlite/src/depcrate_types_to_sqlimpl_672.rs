// Generated macro for impl_672 (impl)
macro_rules! Depcrate_types_to_sqlimpl_672 {
() => {
// Module: crate::types::to_sql
// Provides: {"impl_672"}
// Dependencies: {}
impl < T : ToSql + ? Sized > ToSql for Box < T > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { self . as_ref () . to_sql () } }
};
}
