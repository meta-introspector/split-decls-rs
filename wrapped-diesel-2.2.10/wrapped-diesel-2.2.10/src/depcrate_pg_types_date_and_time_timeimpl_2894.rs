// Generated macro for impl_2894 (impl)
macro_rules! Depcrate_pg_types_date_and_time_timeimpl_2894 {
() => {
// Module: crate::pg::types::date_and_time::time
// Provides: {"impl_2894"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "postgres_backend"))] impl FromSql < Timestamp , Pg > for PrimitiveDateTime { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { let PgTimestamp (offset) = FromSql :: < Timestamp , Pg > :: from_sql (bytes) ? ; match PG_EPOCH . checked_add (Duration :: microseconds (offset)) { Some (v) => Ok (v) , None => { let message = "Tried to deserialize a timestamp that is too large for Time" ; Err (message . into ()) } } } }
};
}
