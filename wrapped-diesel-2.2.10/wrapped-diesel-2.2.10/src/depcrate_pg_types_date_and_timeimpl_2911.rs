// Generated macro for impl_2911 (impl)
macro_rules! Depcrate_pg_types_date_and_timeimpl_2911 {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"impl_2911"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: Timestamp , Pg > for PgTimestamp { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { ToSql :: < sql_types :: BigInt , Pg > :: to_sql (& self . 0 , out) } }
};
}
