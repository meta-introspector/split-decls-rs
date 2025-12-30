// Generated macro for Binary (struct)
macro_rules! Depcrate_sql_typesBinary {
() => {
// Module: crate::sql_types
// Provides: {"Binary"}
// Dependencies: {}
# [doc = " The binary SQL type."] # [doc = ""] # [doc = " Schema inference will treat all variants of `BLOB` as this type (e.g."] # [doc = " `VARBINARY`, `MEDIUMBLOB`, etc)."] # [doc = ""] # [doc = " ### [`ToSql`](crate::serialize::ToSql) impls"] # [doc = ""] # [doc = " - [`Vec<u8>`][Vec]"] # [doc = " - [`&[u8]`][slice]"] # [doc = ""] # [doc = " ### [`FromSql`](crate::deserialize::FromSql) impls"] # [doc = ""] # [doc = " - [`Vec<u8>`][Vec]"] # [doc = ""] # [doc = " [Vec]: std::vec::Vec"] # [doc = " [slice]: https://doc.rust-lang.org/nightly/std/primitive.slice.html"] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (postgres_type (oid = 17 , array_oid = 1001))] # [diesel (sqlite_type (name = "Binary"))] # [diesel (mysql_type (name = "Blob"))] pub struct Binary ;
};
}
