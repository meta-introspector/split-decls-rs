// Generated macro for impl_2895 (impl)
macro_rules! Depcrate_pg_types_date_and_time_timeimpl_2895 {
() => {
// Module: crate::pg::types::date_and_time::time
// Provides: {"impl_2895"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "postgres_backend"))] impl ToSql < Timestamp , Pg > for PrimitiveDateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { let micros = (* self - PG_EPOCH) . whole_microseconds () ; if micros > (i64 :: MAX as i128) { let error_message = format ! ("{self:?} as microseconds is too large to fit in an i64") ; return Err (error_message . into ()) ; } ToSql :: < Timestamp , Pg > :: to_sql (& PgTimestamp (micros . try_into () ?) , & mut out . reborrow ()) } }
};
}
