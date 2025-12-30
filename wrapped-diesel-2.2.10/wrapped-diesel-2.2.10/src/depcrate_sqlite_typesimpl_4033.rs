// Generated macro for impl_4033 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4033 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4033"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl ToSql < sql_types :: Float , Sqlite > for f32 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { out . set_value (* self as f64) ; Ok (IsNull :: No) } }
};
}
