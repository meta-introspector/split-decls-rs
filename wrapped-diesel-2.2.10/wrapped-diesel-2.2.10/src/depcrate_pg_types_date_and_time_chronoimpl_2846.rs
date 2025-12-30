// Generated macro for impl_2846 (impl)
macro_rules! Depcrate_pg_types_date_and_time_chronoimpl_2846 {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"impl_2846"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "postgres_backend"))] impl ToSql < Timestamp , Pg > for NaiveDateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { let time = match (self . signed_duration_since (pg_epoch ())) . num_microseconds () { Some (time) => time , None => { let error_message = format ! ("{self:?} as microseconds is too large to fit in an i64") ; return Err (error_message . into ()) ; } } ; ToSql :: < Timestamp , Pg > :: to_sql (& PgTimestamp (time) , & mut out . reborrow ()) } }
};
}
