// Generated macro for impl_2900 (impl)
macro_rules! Depcrate_pg_types_date_and_time_timeimpl_2900 {
() => {
// Module: crate::pg::types::date_and_time::time
// Provides: {"impl_2900"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "postgres_backend"))] impl ToSql < Time , Pg > for NaiveTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { let duration = * self - NaiveTime :: MIDNIGHT ; ToSql :: < Time , Pg > :: to_sql (& PgTime (duration . whole_microseconds () . try_into () ?) , & mut out . reborrow () ,) } }
};
}
