// Generated macro for impl_84 (impl)
macro_rules! Depcrate_sqliteimpl_84 {
() => {
// Module: crate::sqlite
// Provides: {"impl_84"}
// Dependencies: {}
impl ToSql < sql_types :: Time , Sqlite > for Time { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite > ,) -> serialize :: Result { out . set_value (self . to_jiff () . to_string ()) ; Ok (IsNull :: No) } }
};
}
