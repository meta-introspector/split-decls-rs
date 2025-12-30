// Generated macro for impl_24 (impl)
macro_rules! Depcrate_nullableimpl_24 {
() => {
// Module: crate::nullable
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg (feature = "mysql")] impl < DB : Backend > ToSql < diesel :: sql_types :: Nullable < diesel :: sql_types :: Datetime > , DB > for NullableTimestamp where Option < crate :: Timestamp > : ToSql < diesel :: sql_types :: Nullable < diesel :: sql_types :: Datetime > , DB > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB > ,) -> diesel :: serialize :: Result { self . 0 . to_sql (out) } }
};
}
