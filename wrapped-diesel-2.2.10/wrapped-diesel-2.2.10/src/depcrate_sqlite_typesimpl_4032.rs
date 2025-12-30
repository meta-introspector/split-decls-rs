// Generated macro for impl_4032 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4032 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4032"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl ToSql < sql_types :: BigInt , Sqlite > for i64 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { out . set_value (* self) ; Ok (IsNull :: No) } }
};
}
