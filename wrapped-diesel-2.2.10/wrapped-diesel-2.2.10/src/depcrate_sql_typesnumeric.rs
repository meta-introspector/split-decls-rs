// Generated macro for Numeric (struct)
macro_rules! Depcrate_sql_typesNumeric {
() => {
// Module: crate::sql_types
// Provides: {"Numeric"}
// Dependencies: {}
# [doc = " The arbitrary precision numeric SQL type."] # [doc = ""] # [doc = " This type is only supported on PostgreSQL and MySQL."] # [doc = " On SQLite, [`Double`] should be used instead."] # [doc = ""] # [doc = " ### [`ToSql`](crate::serialize::ToSql) impls"] # [doc = ""] # [doc = " - [`bigdecimal::BigDecimal`] with `feature = [\"numeric\"]`"] # [doc = ""] # [doc = " ### [`FromSql`](crate::deserialize::FromSql) impls"] # [doc = ""] # [doc = " - [`bigdecimal::BigDecimal`] with `feature = [\"numeric\"]`"] # [doc = ""] # [doc = " [`bigdecimal::BigDecimal`]: /bigdecimal/struct.BigDecimal.html"] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (postgres_type (oid = 1700 , array_oid = 1231))] # [diesel (mysql_type (name = "Numeric"))] # [diesel (sqlite_type (name = "Double"))] pub struct Numeric ;
};
}
