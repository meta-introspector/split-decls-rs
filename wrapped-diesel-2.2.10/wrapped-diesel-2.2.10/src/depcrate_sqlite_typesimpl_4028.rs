// Generated macro for impl_4028 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4028 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4028"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl ToSql < sql_types :: Text , Sqlite > for str { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { out . set_value (self) ; Ok (IsNull :: No) } }
};
}
