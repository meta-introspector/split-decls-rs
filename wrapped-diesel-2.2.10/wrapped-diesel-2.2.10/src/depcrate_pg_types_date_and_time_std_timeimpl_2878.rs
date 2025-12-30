// Generated macro for impl_2878 (impl)
macro_rules! Depcrate_pg_types_date_and_time_std_timeimpl_2878 {
() => {
// Module: crate::pg::types::date_and_time::std_time
// Provides: {"impl_2878"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: Timestamp , Pg > for SystemTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { let (before_epoch , duration) = match self . duration_since (pg_epoch ()) { Ok (duration) => (false , duration) , Err (time_err) => (true , time_err . duration ()) , } ; let time_since_epoch = if before_epoch { - (i64 :: try_from (duration_to_usecs (duration)) ?) } else { duration_to_usecs (duration) . try_into () ? } ; ToSql :: < sql_types :: BigInt , Pg > :: to_sql (& time_since_epoch , & mut out . reborrow ()) } }
};
}
