// Generated macro for impl_26 (impl)
macro_rules! Depcrate_nullableimpl_26 {
() => {
// Module: crate::nullable
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl < DB : Backend > ToSql < diesel :: sql_types :: Nullable < diesel :: sql_types :: TimestamptzSqlite > , DB , > for NullableTimestamp where Option < crate :: Timestamp > : ToSql < diesel :: sql_types :: Nullable < diesel :: sql_types :: TimestamptzSqlite > , DB , > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB > ,) -> diesel :: serialize :: Result { self . 0 . to_sql (out) } }
};
}
