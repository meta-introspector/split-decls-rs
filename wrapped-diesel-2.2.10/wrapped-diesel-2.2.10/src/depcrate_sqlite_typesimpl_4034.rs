// Generated macro for impl_4034 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4034 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4034"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl ToSql < sql_types :: Double , Sqlite > for f64 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { out . set_value (* self) ; Ok (IsNull :: No) } }
};
}
