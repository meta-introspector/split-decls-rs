// Generated macro for impl_4092 (impl)
macro_rules! Depcrate_type_impls_primitivesimpl_4092 {
() => {
// Module: crate::type_impls::primitives
// Provides: {"impl_4092"}
// Dependencies: {}
impl < DB > ToSql < sql_types :: Text , DB > for String where DB : Backend , str : ToSql < sql_types :: Text , DB > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB >) -> serialize :: Result { (self as & str) . to_sql (out) } }
};
}
