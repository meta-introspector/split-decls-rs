// Generated macro for impl_2857 (impl)
macro_rules! Depcrate_pg_types_date_and_time_chronoimpl_2857 {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"impl_2857"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "postgres_backend"))] impl FromSql < Date , Pg > for NaiveDate { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { let PgDate (offset) = FromSql :: < Date , Pg > :: from_sql (bytes) ? ; # [allow (deprecated)] let duration = Duration :: days (i64 :: from (offset)) ; match pg_epoch_date () . checked_add_signed (duration) { Some (date) => Ok (date) , None => { let error_message = format ! ("Chrono can only represent dates up to {:?}" , chrono :: NaiveDate :: MAX) ; Err (error_message . into ()) } } } }
};
}
