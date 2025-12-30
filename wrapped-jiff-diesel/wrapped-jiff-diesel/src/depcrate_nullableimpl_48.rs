// Generated macro for impl_48 (impl)
macro_rules! Depcrate_nullableimpl_48 {
() => {
// Module: crate::nullable
// Provides: {"impl_48"}
// Dependencies: {}
impl < DB : Backend , ST > FromSql < ST , DB > for NullableTime where Option < crate :: Time > : FromSql < ST , DB > , { fn from_sql (bytes : < DB as Backend > :: RawValue < '_ > ,) -> diesel :: deserialize :: Result < Self > { FromSql :: from_sql (bytes) . map (NullableTime) } fn from_nullable_sql (bytes : Option < < DB as Backend > :: RawValue < '_ > > ,) -> diesel :: deserialize :: Result < Self > { FromSql :: from_nullable_sql (bytes) . map (NullableTime) } }
};
}
