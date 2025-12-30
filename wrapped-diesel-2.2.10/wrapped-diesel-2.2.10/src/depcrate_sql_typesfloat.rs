// Generated macro for Float (struct)
macro_rules! Depcrate_sql_typesFloat {
() => {
// Module: crate::sql_types
// Provides: {"Float"}
// Dependencies: {}
# [doc = " The float SQL type."] # [doc = ""] # [doc = " ### [`ToSql`](crate::serialize::ToSql) impls"] # [doc = ""] # [doc = " - [`f32`][f32]"] # [doc = ""] # [doc = " ### [`FromSql`](crate::deserialize::FromSql) impls"] # [doc = ""] # [doc = " - [`f32`][f32]"] # [doc = ""] # [doc = " [f32]: https://doc.rust-lang.org/nightly/std/primitive.f32.html"] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (postgres_type (oid = 700 , array_oid = 1021))] # [diesel (sqlite_type (name = "Float"))] # [diesel (mysql_type (name = "Float"))] pub struct Float ;
};
}
