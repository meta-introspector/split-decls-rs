// Generated macro for impl_25 (impl)
macro_rules! Depcrate_nullableimpl_25 {
() => {
// Module: crate::nullable
// Provides: {"impl_25"}
// Dependencies: {}
# [cfg (feature = "postgres")] impl < DB : Backend > ToSql < diesel :: sql_types :: Nullable < diesel :: sql_types :: Timestamptz > , DB > for NullableTimestamp where Option < crate :: Timestamp > : ToSql < diesel :: sql_types :: Nullable < diesel :: sql_types :: Timestamptz > , DB > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB > ,) -> diesel :: serialize :: Result { self . 0 . to_sql (out) } }
};
}
