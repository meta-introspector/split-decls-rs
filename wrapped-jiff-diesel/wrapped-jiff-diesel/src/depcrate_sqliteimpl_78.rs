// Generated macro for impl_78 (impl)
macro_rules! Depcrate_sqliteimpl_78 {
() => {
// Module: crate::sqlite
// Provides: {"impl_78"}
// Dependencies: {}
impl ToSql < sql_types :: TimestamptzSqlite , Sqlite > for Timestamp { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite > ,) -> serialize :: Result { out . set_value (self . to_jiff () . to_string ()) ; Ok (IsNull :: No) } }
};
}
