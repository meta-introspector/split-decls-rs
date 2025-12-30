// Generated macro for impl_47 (impl)
macro_rules! Depcrate_nullableimpl_47 {
() => {
// Module: crate::nullable
// Provides: {"impl_47"}
// Dependencies: {}
impl < DB : Backend > ToSql < diesel :: sql_types :: Nullable < diesel :: sql_types :: Time > , DB > for NullableTime where Option < crate :: Time > : ToSql < diesel :: sql_types :: Nullable < diesel :: sql_types :: Time > , DB > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB > ,) -> diesel :: serialize :: Result { self . 0 . to_sql (out) } }
};
}
