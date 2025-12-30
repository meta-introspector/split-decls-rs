// Generated macro for impl_2856 (impl)
macro_rules! Depcrate_pg_types_date_and_time_chronoimpl_2856 {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"impl_2856"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "postgres_backend"))] impl ToSql < Date , Pg > for NaiveDate { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { let days_since_epoch = self . signed_duration_since (pg_epoch_date ()) . num_days () ; ToSql :: < Date , Pg > :: to_sql (& PgDate (days_since_epoch . try_into () ?) , & mut out . reborrow ()) } }
};
}
