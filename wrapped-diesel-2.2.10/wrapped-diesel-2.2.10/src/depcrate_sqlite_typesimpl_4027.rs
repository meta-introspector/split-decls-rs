// Generated macro for impl_4027 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4027 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4027"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl ToSql < sql_types :: Bool , Sqlite > for bool { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { let int_value = if * self { & 1 } else { & 0 } ; < i32 as ToSql < sql_types :: Integer , Sqlite > > :: to_sql (int_value , out) } }
};
}
