// Generated macro for impl_24 (impl)
macro_rules! Depcrate_serimpl_24 {
() => {
// Module: crate::ser
// Provides: {"impl_24"}
// Dependencies: {}
impl < T > GetExt for Option < T > { type Out = T ; fn right (self) -> Result < Self :: Out , Error > { self . ok_or (Error :: SchemaMismatch) } }
};
}
