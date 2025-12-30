// Generated macro for impl_2904 (impl)
macro_rules! Depcrate_pg_types_date_and_time_timeimpl_2904 {
() => {
// Module: crate::pg::types::date_and_time::time
// Provides: {"impl_2904"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "postgres_backend"))] impl FromSql < Date , Pg > for NaiveDate { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { let PgDate (offset) = FromSql :: < Date , Pg > :: from_sql (bytes) ? ; match PG_EPOCH_DATE . checked_add (Duration :: days (i64 :: from (offset))) { Some (date) => Ok (date) , None => { let error_message = format ! ("Time can only represent dates up to {:?}" , NaiveDate :: MAX) ; Err (error_message . into ()) } } } }
};
}
