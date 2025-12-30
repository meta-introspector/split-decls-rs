// Generated macro for impl_2897 (impl)
macro_rules! Depcrate_pg_types_date_and_time_timeimpl_2897 {
() => {
// Module: crate::pg::types::date_and_time::time
// Provides: {"impl_2897"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "postgres_backend"))] impl ToSql < Timestamptz , Pg > for PrimitiveDateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { ToSql :: < Timestamp , Pg > :: to_sql (self , out) } }
};
}
