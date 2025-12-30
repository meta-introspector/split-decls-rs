// Generated macro for impl_4071 (impl)
macro_rules! Depcrate_type_impls_optionimpl_4071 {
() => {
// Module: crate::type_impls::option
// Provides: {"impl_4071"}
// Dependencies: {}
impl < T , ST , DB > FromSql < Nullable < ST > , DB > for Option < T > where T : FromSql < ST , DB > , DB : Backend , ST : SqlType < IsNull = is_nullable :: NotNull > , { fn from_sql (bytes : DB :: RawValue < '_ >) -> deserialize :: Result < Self > { T :: from_sql (bytes) . map (Some) } fn from_nullable_sql (bytes : Option < DB :: RawValue < '_ > >) -> deserialize :: Result < Self > { match bytes { Some (bytes) => T :: from_sql (bytes) . map (Some) , None => Ok (None) , } } }
};
}
