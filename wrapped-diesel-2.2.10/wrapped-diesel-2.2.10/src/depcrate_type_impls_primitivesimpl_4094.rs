// Generated macro for impl_4094 (impl)
macro_rules! Depcrate_type_impls_primitivesimpl_4094 {
() => {
// Module: crate::type_impls::primitives
// Provides: {"impl_4094"}
// Dependencies: {}
impl < DB > ToSql < sql_types :: Binary , DB > for Vec < u8 > where DB : Backend , [u8] : ToSql < sql_types :: Binary , DB > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB >) -> serialize :: Result { (self as & [u8]) . to_sql (out) } }
};
}
