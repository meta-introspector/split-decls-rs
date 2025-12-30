// Generated macro for impl_4095 (impl)
macro_rules! Depcrate_type_impls_primitivesimpl_4095 {
() => {
// Module: crate::type_impls::primitives
// Provides: {"impl_4095"}
// Dependencies: {}
impl < DB , const N : usize > ToSql < sql_types :: Binary , DB > for [u8 ; N] where DB : Backend , [u8] : ToSql < sql_types :: Binary , DB > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB >) -> serialize :: Result { self . as_slice () . to_sql (out) } }
};
}
