// Generated macro for PgDate (struct)
macro_rules! Depcrate_pg_types_date_and_timePgDate {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"PgDate"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , AsExpression , FromSqlRow)] # [diesel (sql_type = Date)] # [doc = " Dates are represented in Postgres as a 32 bit signed integer representing the number of julian"] # [doc = " days since January 1st 2000. This struct is a dumb wrapper type, meant only to indicate the"] # [doc = " integer's meaning."] pub struct PgDate (pub i32) ;
};
}
