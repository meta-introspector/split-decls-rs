// Generated macro for PgInterval (struct)
macro_rules! Depcrate_pg_types_date_and_timePgInterval {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"PgInterval"}
// Dependencies: {}
# [doc = " Intervals in Postgres are separated into 3 parts. A 64 bit integer representing time in"] # [doc = " microseconds, a 32 bit integer representing number of days, and a 32 bit integer"] # [doc = " representing number of months. This struct is a dumb wrapper type, meant only to indicate the"] # [doc = " meaning of these parts."] # [cfg (feature = "postgres_backend")] # [derive (Debug , Clone , Copy , PartialEq , Eq , AsExpression , FromSqlRow)] # [diesel (sql_type = Interval)] pub struct PgInterval { # [doc = " The number of whole microseconds"] pub microseconds : i64 , # [doc = " The number of whole days"] pub days : i32 , # [doc = " The number of whole months"] pub months : i32 , }
};
}
