// Generated macro for impl_2861 (impl)
macro_rules! Depcrate_pg_types_date_and_time_chronoimpl_2861 {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"impl_2861"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "postgres_backend"))] impl ToSql < Interval , Pg > for Duration { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { let microseconds : i64 = if let Some (v) = self . num_microseconds () { v % (MICROSECONDS_PER_SECOND * SECONDS_PER_DAY) } else { return Err ("Failed to create microseconds by overflow" . into ()) ; } ; let days : i32 = self . num_days () . try_into () . expect ("Failed to get i32 days from i64") ; let interval = PgInterval { microseconds , days , months : 0 , } ; < PgInterval as ToSql < Interval , Pg > > :: to_sql (& interval , & mut out . reborrow ()) } }
};
}
