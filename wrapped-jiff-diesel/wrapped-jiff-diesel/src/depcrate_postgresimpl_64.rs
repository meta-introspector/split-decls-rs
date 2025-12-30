// Generated macro for impl_64 (impl)
macro_rules! Depcrate_postgresimpl_64 {
() => {
// Module: crate::postgres
// Provides: {"impl_64"}
// Dependencies: {}
impl ToSql < sql_types :: Timestamptz , Pg > for Timestamp { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg > ,) -> serialize :: Result { let dt = UTC . to_datetime (self . to_jiff ()) . to_diesel () ; ToSql :: < sql_types :: Timestamp , Pg > :: to_sql (& dt , & mut out . reborrow ()) } }
};
}
