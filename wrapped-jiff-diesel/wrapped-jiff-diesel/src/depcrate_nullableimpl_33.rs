// Generated macro for impl_33 (impl)
macro_rules! Depcrate_nullableimpl_33 {
() => {
// Module: crate::nullable
// Provides: {"impl_33"}
// Dependencies: {}
impl < DB : Backend > ToSql < diesel :: sql_types :: Nullable < diesel :: sql_types :: Timestamp > , DB > for NullableDateTime where Option < crate :: DateTime > : ToSql < diesel :: sql_types :: Nullable < diesel :: sql_types :: Timestamp > , DB > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB > ,) -> diesel :: serialize :: Result { self . 0 . to_sql (out) } }
};
}
