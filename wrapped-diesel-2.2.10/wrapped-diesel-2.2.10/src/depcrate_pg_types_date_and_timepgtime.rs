// Generated macro for PgTime (struct)
macro_rules! Depcrate_pg_types_date_and_timePgTime {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"PgTime"}
// Dependencies: {}
# [doc = " Time is represented in Postgres as a 64 bit signed integer representing the number of"] # [doc = " microseconds since midnight. This struct is a dumb wrapper type, meant only to indicate the"] # [doc = " integer's meaning."] # [cfg (feature = "postgres_backend")] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , AsExpression , FromSqlRow)] # [diesel (sql_type = Time)] pub struct PgTime (pub i64) ;
};
}
