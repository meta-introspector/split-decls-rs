// Generated macro for Date (struct)
macro_rules! Depcrate_sql_typesDate {
() => {
// Module: crate::sql_types
// Provides: {"Date"}
// Dependencies: {}
# [doc = " The date SQL type."] # [doc = ""] # [doc = " ### [`ToSql`](crate::serialize::ToSql) impls"] # [doc = ""] # [doc = " - [`chrono::NaiveDate`][NaiveDate] with `feature = \"chrono\"`"] # [doc = " - [`time::Date`][Date] with `feature = \"time\"`"] # [doc = ""] # [doc = " ### [`FromSql`](crate::deserialize::FromSql) impls"] # [doc = ""] # [doc = " - [`chrono::NaiveDate`][NaiveDate] with `feature = \"chrono\"`"] # [doc = " - [`time::Date`][Date] with `feature = \"time\"`"] # [doc = ""] # [doc = " [NaiveDate]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveDate.html"] # [doc = " [Date]: https://docs.rs/time/0.3.9/time/struct.Date.html"] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (postgres_type (oid = 1082 , array_oid = 1182))] # [diesel (sqlite_type (name = "Text"))] # [diesel (mysql_type (name = "Date"))] pub struct Date ;
};
}
