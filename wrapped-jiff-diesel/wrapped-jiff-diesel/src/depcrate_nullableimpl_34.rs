// Generated macro for impl_34 (impl)
macro_rules! Depcrate_nullableimpl_34 {
() => {
// Module: crate::nullable
// Provides: {"impl_34"}
// Dependencies: {}
impl < DB : Backend , ST > FromSql < ST , DB > for NullableDateTime where Option < crate :: DateTime > : FromSql < ST , DB > , { fn from_sql (bytes : < DB as Backend > :: RawValue < '_ > ,) -> diesel :: deserialize :: Result < Self > { FromSql :: from_sql (bytes) . map (NullableDateTime) } fn from_nullable_sql (bytes : Option < < DB as Backend > :: RawValue < '_ > > ,) -> diesel :: deserialize :: Result < Self > { FromSql :: from_nullable_sql (bytes) . map (NullableDateTime) } }
};
}
