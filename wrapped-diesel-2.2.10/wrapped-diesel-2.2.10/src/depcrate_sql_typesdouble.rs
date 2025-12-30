// Generated macro for Double (struct)
macro_rules! Depcrate_sql_typesDouble {
() => {
// Module: crate::sql_types
// Provides: {"Double"}
// Dependencies: {}
# [doc = " The double precision float SQL type."] # [doc = ""] # [doc = " ### [`ToSql`](crate::serialize::ToSql) impls"] # [doc = ""] # [doc = " - [`f64`][f64]"] # [doc = ""] # [doc = " ### [`FromSql`](crate::deserialize::FromSql) impls"] # [doc = ""] # [doc = " - [`f64`][f64]"] # [doc = ""] # [doc = " [f64]: https://doc.rust-lang.org/nightly/std/primitive.f64.html"] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (postgres_type (oid = 701 , array_oid = 1022))] # [diesel (sqlite_type (name = "Double"))] # [diesel (mysql_type (name = "Double"))] pub struct Double ;
};
}
