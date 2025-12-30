// Generated macro for impl_66 (impl)
macro_rules! Depcrate_postgresimpl_66 {
() => {
// Module: crate::postgres
// Provides: {"impl_66"}
// Dependencies: {}
impl ToSql < sql_types :: Timestamp , Pg > for DateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg > ,) -> serialize :: Result { let micros = self . to_jiff () . duration_since (POSTGRES_EPOCH_DATETIME) . as_micros () ; let micros = i64 :: try_from (micros) . unwrap () ; ToSql :: < sql_types :: Timestamp , Pg > :: to_sql (& PgTimestamp (micros) , & mut out . reborrow () ,) } }
};
}
