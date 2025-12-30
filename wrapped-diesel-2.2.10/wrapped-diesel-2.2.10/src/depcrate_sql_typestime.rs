// Generated macro for Time (struct)
macro_rules! Depcrate_sql_typesTime {
() => {
// Module: crate::sql_types
// Provides: {"Time"}
// Dependencies: {}
# [doc = " The time SQL type."] # [doc = ""] # [doc = " ### [`ToSql`](crate::serialize::ToSql) impls"] # [doc = ""] # [doc = " - [`chrono::NaiveTime`][NaiveTime] with `feature = \"chrono\"`"] # [doc = " - [`time::Time`][Time] with `feature = \"time\"`"] # [doc = ""] # [doc = " ### [`FromSql`](crate::deserialize::FromSql) impls"] # [doc = ""] # [doc = " - [`chrono::NaiveTime`][NaiveTime] with `feature = \"chrono\"`"] # [doc = " - [`time::Time`][Time] with `feature = \"time\"`"] # [doc = ""] # [doc = " [NaiveTime]: /chrono/naive/time/struct.NaiveTime.html"] # [doc = " [Time]: /time/struct.Time.html"] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (postgres_type (oid = 1083 , array_oid = 1183))] # [diesel (sqlite_type (name = "Text"))] # [diesel (mysql_type (name = "Time"))] pub struct Time ;
};
}
