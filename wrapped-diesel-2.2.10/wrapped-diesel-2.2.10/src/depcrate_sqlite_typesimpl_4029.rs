// Generated macro for impl_4029 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4029 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4029"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl ToSql < sql_types :: Binary , Sqlite > for [u8] { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { out . set_value (self) ; Ok (IsNull :: No) } }
};
}
