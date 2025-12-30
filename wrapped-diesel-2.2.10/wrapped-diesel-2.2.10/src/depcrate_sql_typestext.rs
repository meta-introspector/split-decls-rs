// Generated macro for Text (struct)
macro_rules! Depcrate_sql_typesText {
() => {
// Module: crate::sql_types
// Provides: {"Text"}
// Dependencies: {}
# [doc = " The text SQL type."] # [doc = ""] # [doc = " On all backends strings must be valid UTF-8."] # [doc = " On PostgreSQL strings must not include nul bytes."] # [doc = ""] # [doc = " Schema inference will treat all variants of `TEXT` as this type (e.g."] # [doc = " `VARCHAR`, `MEDIUMTEXT`, etc)."] # [doc = ""] # [doc = " ### [`ToSql`](crate::serialize::ToSql) impls"] # [doc = ""] # [doc = " - [`String`]"] # [doc = " - [`&str`][str]"] # [doc = ""] # [doc = " ### [`FromSql`](crate::deserialize::FromSql) impls"] # [doc = ""] # [doc = " - [`String`]"] # [doc = ""] # [doc = " [str]: https://doc.rust-lang.org/nightly/std/primitive.str.html"] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (postgres_type (oid = 25 , array_oid = 1009))] # [diesel (sqlite_type (name = "Text"))] # [diesel (mysql_type (name = "String"))] pub struct Text ;
};
}
