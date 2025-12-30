// Generated macro for impl_2903 (impl)
macro_rules! Depcrate_pg_types_date_and_time_timeimpl_2903 {
() => {
// Module: crate::pg::types::date_and_time::time
// Provides: {"impl_2903"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "postgres_backend"))] impl ToSql < Date , Pg > for NaiveDate { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { let days_since_epoch = (* self - PG_EPOCH_DATE) . whole_days () ; ToSql :: < Date , Pg > :: to_sql (& PgDate (days_since_epoch . try_into () ?) , & mut out . reborrow ()) } }
};
}
