// Generated macro for impl_9 (impl)
macro_rules! Depcrate_deimpl_9 {
() => {
// Module: crate::de
// Provides: {"impl_9"}
// Dependencies: {}
impl < T > GetExt for Option < T > { type Out = T ; fn right (self) -> Result < Self :: Out , Error > { self . ok_or (Error :: SchemaMismatch) } }
};
}
