// Generated macro for impl_4093 (impl)
macro_rules! Depcrate_type_impls_primitivesimpl_4093 {
() => {
// Module: crate::type_impls::primitives
// Provides: {"impl_4093"}
// Dependencies: {}
impl < ST , DB > FromSql < ST , DB > for Vec < u8 > where DB : Backend , * const [u8] : FromSql < ST , DB > , { # [allow (unsafe_code)] fn from_sql (bytes : DB :: RawValue < '_ >) -> deserialize :: Result < Self > { let slice_ptr = < * const [u8] as FromSql < ST , DB > > :: from_sql (bytes) ? ; let bytes = unsafe { & * slice_ptr } ; Ok (bytes . to_owned ()) } }
};
}
