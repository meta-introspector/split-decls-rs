// Generated macro for impl_2915 (impl)
macro_rules! Depcrate_pg_types_date_and_timeimpl_2915 {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"impl_2915"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: Date , Pg > for PgDate { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { ToSql :: < sql_types :: Integer , Pg > :: to_sql (& self . 0 , out) } }
};
}
