// Generated macro for impl_2913 (impl)
macro_rules! Depcrate_pg_types_date_and_timeimpl_2913 {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"impl_2913"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: Timestamptz , Pg > for PgTimestamp { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { ToSql :: < sql_types :: Timestamp , Pg > :: to_sql (self , out) } }
};
}
