// Generated macro for Bool (struct)
macro_rules! Depcrate_sql_typesBool {
() => {
// Module: crate::sql_types
// Provides: {"Bool"}
// Dependencies: {}
# [doc = " The boolean SQL type."] # [doc = ""] # [doc = " On backends without a native boolean type,"] # [doc = " this is emulated with the smallest supported integer."] # [doc = ""] # [doc = " ### [`ToSql`](crate::serialize::ToSql) impls"] # [doc = ""] # [doc = " - [`bool`][bool]"] # [doc = ""] # [doc = " ### [`FromSql`](crate::deserialize::FromSql) impls"] # [doc = ""] # [doc = " - [`bool`][bool]"] # [doc = ""] # [doc = " [bool]: https://doc.rust-lang.org/nightly/std/primitive.bool.html"] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (postgres_type (oid = 16 , array_oid = 1000))] # [diesel (sqlite_type (name = "Integer"))] # [diesel (mysql_type (name = "Tiny"))] pub struct Bool ;
};
}
