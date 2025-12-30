// Generated macro for impl_4098 (impl)
macro_rules! Depcrate_type_impls_primitivesimpl_4098 {
() => {
// Module: crate::type_impls::primitives
// Provides: {"impl_4098"}
// Dependencies: {}
impl < 'a , T : ? Sized , ST , DB > FromSql < ST , DB > for Cow < 'a , T > where T : 'a + ToOwned , DB : Backend , T :: Owned : FromSql < ST , DB > , { fn from_sql (bytes : DB :: RawValue < '_ >) -> deserialize :: Result < Self > { T :: Owned :: from_sql (bytes) . map (Cow :: Owned) } }
};
}
