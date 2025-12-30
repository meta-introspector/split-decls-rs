// Generated macro for impl_669 (impl)
macro_rules! Depcrate_types_to_sqlimpl_669 {
() => {
// Module: crate::types::to_sql
// Provides: {"impl_669"}
// Dependencies: {}
impl ToSql for ToSqlOutput < '_ > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (match * self { ToSqlOutput :: Borrowed (v) => ToSqlOutput :: Borrowed (v) , ToSqlOutput :: Owned (ref v) => ToSqlOutput :: Borrowed (ValueRef :: from (v)) , # [cfg (feature = "blob")] ToSqlOutput :: ZeroBlob (i) => ToSqlOutput :: ZeroBlob (i) , # [cfg (feature = "functions")] ToSqlOutput :: Arg (i) => ToSqlOutput :: Arg (i) , # [cfg (feature = "array")] ToSqlOutput :: Array (ref a) => ToSqlOutput :: Array (a . clone ()) , }) } }
};
}
