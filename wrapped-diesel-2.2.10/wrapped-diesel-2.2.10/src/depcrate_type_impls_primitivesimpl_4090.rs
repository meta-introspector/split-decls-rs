// Generated macro for impl_4090 (impl)
macro_rules! Depcrate_type_impls_primitivesimpl_4090 {
() => {
// Module: crate::type_impls::primitives
// Provides: {"impl_4090"}
// Dependencies: {}
impl < ST , DB > FromSql < ST , DB > for String where DB : Backend , * const str : FromSql < ST , DB > , { # [allow (unsafe_code)] fn from_sql (bytes : DB :: RawValue < '_ >) -> deserialize :: Result < Self > { let str_ptr = < * const str as FromSql < ST , DB > > :: from_sql (bytes) ? ; let string = unsafe { & * str_ptr } ; Ok (string . to_owned ()) } }
};
}
