// Generated macro for impl_3992 (impl)
macro_rules! Depcrate_sqlite_types_date_and_timeimpl_3992 {
() => {
// Module: crate::sqlite::types::date_and_time
// Provides: {"impl_3992"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl ToSql < sql_types :: Date , Sqlite > for String { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { < str as ToSql < sql_types :: Date , Sqlite > > :: to_sql (self as & str , out) } }
};
}
