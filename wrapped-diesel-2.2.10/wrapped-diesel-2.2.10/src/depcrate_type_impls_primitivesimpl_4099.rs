// Generated macro for impl_4099 (impl)
macro_rules! Depcrate_type_impls_primitivesimpl_4099 {
() => {
// Module: crate::type_impls::primitives
// Provides: {"impl_4099"}
// Dependencies: {}
impl < 'a , T : ? Sized , ST , DB > Queryable < ST , DB > for Cow < 'a , T > where T : 'a + ToOwned , ST : SingleValue , DB : Backend , Self : FromSql < ST , DB > , { type Row = Self ; fn build (row : Self :: Row) -> deserialize :: Result < Self > { Ok (row) } }
};
}
