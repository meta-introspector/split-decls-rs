// Generated macro for SmallInt (struct)
macro_rules! Depcrate_sql_typesSmallInt {
() => {
// Module: crate::sql_types
// Provides: {"SmallInt"}
// Dependencies: {}
# [doc = " The small integer SQL type."] # [doc = ""] # [doc = " ### [`ToSql`](crate::serialize::ToSql) impls"] # [doc = ""] # [doc = " - [`i16`][i16]"] # [doc = ""] # [doc = " ### [`FromSql`](crate::deserialize::FromSql) impls"] # [doc = ""] # [doc = " - [`i16`][i16]"] # [doc = ""] # [doc = " [i16]: https://doc.rust-lang.org/nightly/std/primitive.i16.html"] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (postgres_type (oid = 21 , array_oid = 1005))] # [diesel (sqlite_type (name = "SmallInt"))] # [diesel (mysql_type (name = "Short"))] pub struct SmallInt ;
};
}
