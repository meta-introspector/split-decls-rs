// Generated macro for Interval (struct)
macro_rules! Depcrate_sql_typesInterval {
() => {
// Module: crate::sql_types
// Provides: {"Interval"}
// Dependencies: {}
# [doc = " The interval SQL type."] # [doc = ""] # [doc = " This type is currently only implemented for PostgreSQL."] # [doc = ""] # [doc = " ### [`ToSql`](crate::serialize::ToSql) impls"] # [doc = ""] # [doc = " - [`PgInterval`] which can be constructed using [`IntervalDsl`]"] # [doc = " - [`chrono::Duration`][Duration] with `feature = \"chrono\"`"] # [doc = ""] # [doc = " ### [`FromSql`](crate::deserialize::FromSql) impls"] # [doc = ""] # [doc = " - [`PgInterval`] which can be constructed using [`IntervalDsl`]"] # [doc = " - [`chrono::Duration`][Duration] with `feature = \"chrono\"`"] # [doc = "   (There might be some information loss due to special behavior for literal `month` (or longer) intervals;"] # [doc = "   Please read official documentation of [PostgreSQL Interval].)"] # [doc = ""] # [doc = " [`PgInterval`]: ../pg/data_types/struct.PgInterval.html"] # [doc = " [`IntervalDsl`]: ../pg/expression/extensions/trait.IntervalDsl.html"] # [doc = " [Duration]: https://docs.rs/chrono/*/chrono/type.Duration.html"] # [doc = " [PostgreSQL Interval]: https://www.postgresql.org/docs/current/datatype-datetime.html#DATATYPE-INTERVAL-INPUT"] # [derive (Debug , Clone , Copy , Default , QueryId , SqlType)] # [diesel (postgres_type (oid = 1186 , array_oid = 1187))] pub struct Interval ;
};
}
