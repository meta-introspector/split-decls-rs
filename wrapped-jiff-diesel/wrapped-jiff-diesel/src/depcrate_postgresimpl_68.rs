// Generated macro for impl_68 (impl)
macro_rules! Depcrate_postgresimpl_68 {
() => {
// Module: crate::postgres
// Provides: {"impl_68"}
// Dependencies: {}
impl ToSql < sql_types :: Date , Pg > for Date { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg > ,) -> serialize :: Result { let days = (self . to_jiff () - POSTGRES_EPOCH_DATE) . get_days () ; ToSql :: < sql_types :: Date , Pg > :: to_sql (& PgDate (days) , & mut out . reborrow () ,) } }
};
}
