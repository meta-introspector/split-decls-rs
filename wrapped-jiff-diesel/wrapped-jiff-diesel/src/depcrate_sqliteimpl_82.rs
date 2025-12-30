// Generated macro for impl_82 (impl)
macro_rules! Depcrate_sqliteimpl_82 {
() => {
// Module: crate::sqlite
// Provides: {"impl_82"}
// Dependencies: {}
impl ToSql < sql_types :: Date , Sqlite > for Date { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite > ,) -> serialize :: Result { out . set_value (self . to_jiff () . to_string ()) ; Ok (IsNull :: No) } }
};
}
