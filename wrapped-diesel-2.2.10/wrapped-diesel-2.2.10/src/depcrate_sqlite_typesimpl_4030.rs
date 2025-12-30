// Generated macro for impl_4030 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4030 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4030"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl ToSql < sql_types :: SmallInt , Sqlite > for i16 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { out . set_value (* self as i32) ; Ok (IsNull :: No) } }
};
}
