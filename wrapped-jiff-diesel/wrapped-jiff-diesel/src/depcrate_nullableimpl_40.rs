// Generated macro for impl_40 (impl)
macro_rules! Depcrate_nullableimpl_40 {
() => {
// Module: crate::nullable
// Provides: {"impl_40"}
// Dependencies: {}
impl < DB : Backend > ToSql < diesel :: sql_types :: Nullable < diesel :: sql_types :: Date > , DB > for NullableDate where Option < crate :: Date > : ToSql < diesel :: sql_types :: Nullable < diesel :: sql_types :: Date > , DB > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB > ,) -> diesel :: serialize :: Result { self . 0 . to_sql (out) } }
};
}
