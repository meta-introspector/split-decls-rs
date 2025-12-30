// Generated macro for impl_109 (impl)
macro_rules! Depcrate_provider_exceptions_builderimpl_109 {
() => {
// Module: crate::provider::exceptions_builder
// Provides: {"impl_109"}
// Dependencies: {}
impl AsULE for ExceptionHeader { type ULE = ExceptionHeaderULE ; fn from_unaligned (u : ExceptionHeaderULE) -> Self { Self { slot_presence : u . slot_presence , bits : ExceptionBits :: from_integer (u . bits . 0) , } } fn to_unaligned (self) -> ExceptionHeaderULE { ExceptionHeaderULE { slot_presence : self . slot_presence , bits : ExceptionBitsULE (self . bits . to_integer ()) , } } }
};
}
