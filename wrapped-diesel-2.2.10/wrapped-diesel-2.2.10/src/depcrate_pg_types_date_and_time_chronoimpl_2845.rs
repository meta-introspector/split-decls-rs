// Generated macro for impl_2845 (impl)
macro_rules! Depcrate_pg_types_date_and_time_chronoimpl_2845 {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"impl_2845"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "postgres_backend"))] impl FromSql < Timestamp , Pg > for NaiveDateTime { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { let PgTimestamp (offset) = FromSql :: < Timestamp , Pg > :: from_sql (bytes) ? ; match pg_epoch () . checked_add_signed (Duration :: microseconds (offset)) { Some (v) => Ok (v) , None => { let message = "Tried to deserialize a timestamp that is too large for Chrono" ; Err (message . into ()) } } } }
};
}
