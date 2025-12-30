// Generated macro for impl_2853 (impl)
macro_rules! Depcrate_pg_types_date_and_time_chronoimpl_2853 {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"impl_2853"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "postgres_backend"))] impl ToSql < Time , Pg > for NaiveTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { let duration = self . signed_duration_since (midnight ()) ; match duration . num_microseconds () { Some (offset) => ToSql :: < Time , Pg > :: to_sql (& PgTime (offset) , & mut out . reborrow ()) , None => unreachable ! () , } } }
};
}
