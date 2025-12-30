// Generated macro for impl_70 (impl)
macro_rules! Depcrate_postgresimpl_70 {
() => {
// Module: crate::postgres
// Provides: {"impl_70"}
// Dependencies: {}
impl ToSql < sql_types :: Time , Pg > for Time { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg > ,) -> serialize :: Result { let micros = self . to_jiff () . duration_since (MIDNIGHT) . as_micros () ; let micros = i64 :: try_from (micros) . unwrap () ; ToSql :: < sql_types :: Time , Pg > :: to_sql (& PgTime (micros) , & mut out . reborrow () ,) } }
};
}
