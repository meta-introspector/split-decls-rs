// Generated macro for impl_4031 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4031 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4031"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl ToSql < sql_types :: Integer , Sqlite > for i32 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { out . set_value (* self) ; Ok (IsNull :: No) } }
};
}
