// Generated macro for impl_3995 (impl)
macro_rules! Depcrate_sqlite_types_date_and_timeimpl_3995 {
() => {
// Module: crate::sqlite::types::date_and_time
// Provides: {"impl_3995"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl ToSql < sql_types :: Time , Sqlite > for String { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { < str as ToSql < sql_types :: Time , Sqlite > > :: to_sql (self as & str , out) } }
};
}
