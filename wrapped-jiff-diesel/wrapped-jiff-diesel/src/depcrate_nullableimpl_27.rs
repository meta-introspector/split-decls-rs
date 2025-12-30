// Generated macro for impl_27 (impl)
macro_rules! Depcrate_nullableimpl_27 {
() => {
// Module: crate::nullable
// Provides: {"impl_27"}
// Dependencies: {}
impl < DB : Backend , ST > FromSql < ST , DB > for NullableTimestamp where Option < crate :: Timestamp > : FromSql < ST , DB > , { fn from_sql (bytes : < DB as Backend > :: RawValue < '_ > ,) -> diesel :: deserialize :: Result < Self > { FromSql :: from_sql (bytes) . map (NullableTimestamp) } fn from_nullable_sql (bytes : Option < < DB as Backend > :: RawValue < '_ > > ,) -> diesel :: deserialize :: Result < Self > { FromSql :: from_nullable_sql (bytes) . map (NullableTimestamp) } }
};
}
