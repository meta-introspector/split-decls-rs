// Generated macro for impl_2848 (impl)
macro_rules! Depcrate_pg_types_date_and_time_chronoimpl_2848 {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"impl_2848"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "postgres_backend"))] impl ToSql < Timestamptz , Pg > for NaiveDateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { ToSql :: < Timestamp , Pg > :: to_sql (self , out) } }
};
}
