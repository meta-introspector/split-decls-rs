// Generated macro for impl_2919 (impl)
macro_rules! Depcrate_pg_types_date_and_timeimpl_2919 {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"impl_2919"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: Interval , Pg > for PgInterval { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { ToSql :: < sql_types :: BigInt , Pg > :: to_sql (& self . microseconds , out) ? ; ToSql :: < sql_types :: Integer , Pg > :: to_sql (& self . days , out) ? ; ToSql :: < sql_types :: Integer , Pg > :: to_sql (& self . months , out) ? ; Ok (IsNull :: No) } }
};
}
