// Generated macro for Integer (struct)
macro_rules! Depcrate_sql_typesInteger {
() => {
// Module: crate::sql_types
// Provides: {"Integer"}
// Dependencies: {}
# [doc = " The integer SQL type."] # [doc = ""] # [doc = " ### [`ToSql`](crate::serialize::ToSql) impls"] # [doc = ""] # [doc = " - [`i32`][i32]"] # [doc = ""] # [doc = " ### [`FromSql`](crate::deserialize::FromSql) impls"] # [doc = ""] # [doc = " - [`i32`][i32]"] # [doc = ""] # [doc = " [i32]: https://doc.rust-lang.org/nightly/std/primitive.i32.html"] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (postgres_type (oid = 23 , array_oid = 1007))] # [diesel (sqlite_type (name = "Integer"))] # [diesel (mysql_type (name = "Long"))] pub struct Integer ;
};
}
