// Generated macro for impl_882 (impl)
macro_rules! Depcrate_vtab_arrayimpl_882 {
() => {
// Module: crate::vtab::array
// Provides: {"impl_882"}
// Dependencies: {}
impl ToSql for Array { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: Array (self . clone ())) } }
};
}
