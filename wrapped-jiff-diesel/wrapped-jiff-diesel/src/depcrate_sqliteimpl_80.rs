// Generated macro for impl_80 (impl)
macro_rules! Depcrate_sqliteimpl_80 {
() => {
// Module: crate::sqlite
// Provides: {"impl_80"}
// Dependencies: {}
impl ToSql < sql_types :: Timestamp , Sqlite > for DateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite > ,) -> serialize :: Result { out . set_value (self . to_jiff () . to_string ()) ; Ok (IsNull :: No) } }
};
}
