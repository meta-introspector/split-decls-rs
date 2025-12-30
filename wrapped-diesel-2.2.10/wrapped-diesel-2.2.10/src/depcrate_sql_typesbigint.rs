// Generated macro for BigInt (struct)
macro_rules! Depcrate_sql_typesBigInt {
() => {
// Module: crate::sql_types
// Provides: {"BigInt"}
// Dependencies: {}
# [doc = " The big integer SQL type."] # [doc = ""] # [doc = " ### [`ToSql`](crate::serialize::ToSql) impls"] # [doc = ""] # [doc = " - [`i64`][i64]"] # [doc = ""] # [doc = " ### [`FromSql`](crate::deserialize::FromSql) impls"] # [doc = ""] # [doc = " - [`i64`][i64]"] # [doc = ""] # [doc = " [i64]: https://doc.rust-lang.org/nightly/std/primitive.i64.html"] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (postgres_type (oid = 20 , array_oid = 1016))] # [diesel (sqlite_type (name = "Long"))] # [diesel (mysql_type (name = "LongLong"))] pub struct BigInt ;
};
}
