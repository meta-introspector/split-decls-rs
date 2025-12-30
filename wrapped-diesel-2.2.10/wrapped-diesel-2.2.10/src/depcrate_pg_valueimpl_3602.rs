// Generated macro for impl_3602 (impl)
macro_rules! Depcrate_pg_valueimpl_3602 {
() => {
// Module: crate::pg::value
// Provides: {"impl_3602"}
// Dependencies: {}
impl < F > TypeOidLookup for F where F : Fn () -> NonZeroU32 , { fn lookup (& self) -> NonZeroU32 { (self) () } }
};
}
