// Generated macro for impl_2851 (impl)
macro_rules! Depcrate_pg_types_date_and_time_chronoimpl_2851 {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"impl_2851"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "postgres_backend"))] impl < TZ : TimeZone > ToSql < Timestamptz , Pg > for DateTime < TZ > { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { ToSql :: < Timestamptz , Pg > :: to_sql (& self . naive_utc () , & mut out . reborrow ()) } }
};
}
