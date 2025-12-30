// Generated macro for PgTimestamp (struct)
macro_rules! Depcrate_pg_types_date_and_timePgTimestamp {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"PgTimestamp"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , AsExpression , FromSqlRow)] # [diesel (sql_type = Timestamp)] # [diesel (sql_type = Timestamptz)] # [doc = " Timestamps are represented in Postgres as a 64 bit signed integer representing the number of"] # [doc = " microseconds since January 1st 2000. This struct is a dumb wrapper type, meant only to indicate"] # [doc = " the integer's meaning."] pub struct PgTimestamp (pub i64) ;
};
}
