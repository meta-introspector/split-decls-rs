// Generated macro for impl_54 (impl)
macro_rules! Depcrate_nullableimpl_54 {
() => {
// Module: crate::nullable
// Provides: {"impl_54"}
// Dependencies: {}
impl < DB : Backend , ST > FromSql < ST , DB > for NullableSpan where Option < crate :: Span > : FromSql < ST , DB > , { fn from_sql (bytes : < DB as Backend > :: RawValue < '_ > ,) -> diesel :: deserialize :: Result < Self > { FromSql :: from_sql (bytes) . map (NullableSpan) } fn from_nullable_sql (bytes : Option < < DB as Backend > :: RawValue < '_ > > ,) -> diesel :: deserialize :: Result < Self > { FromSql :: from_nullable_sql (bytes) . map (NullableSpan) } }
};
}
