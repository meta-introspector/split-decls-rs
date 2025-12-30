// Generated macro for impl_41 (impl)
macro_rules! Depcrate_nullableimpl_41 {
() => {
// Module: crate::nullable
// Provides: {"impl_41"}
// Dependencies: {}
impl < DB : Backend , ST > FromSql < ST , DB > for NullableDate where Option < crate :: Date > : FromSql < ST , DB > , { fn from_sql (bytes : < DB as Backend > :: RawValue < '_ > ,) -> diesel :: deserialize :: Result < Self > { FromSql :: from_sql (bytes) . map (NullableDate) } fn from_nullable_sql (bytes : Option < < DB as Backend > :: RawValue < '_ > > ,) -> diesel :: deserialize :: Result < Self > { FromSql :: from_nullable_sql (bytes) . map (NullableDate) } }
};
}
