// Generated macro for impl_673 (impl)
macro_rules! Depcrate_types_to_sqlimpl_673 {
() => {
// Module: crate::types::to_sql
// Provides: {"impl_673"}
// Dependencies: {}
impl < T : ToSql + ? Sized > ToSql for std :: rc :: Rc < T > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { self . as_ref () . to_sql () } }
};
}
