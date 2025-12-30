// Generated macro for impl_70 (impl)
macro_rules! Depcrate_provider_exception_helpersimpl_70 {
() => {
// Module: crate::provider::exception_helpers
// Provides: {"impl_70"}
// Dependencies: {}
impl AsULE for ExceptionBits { type ULE = ExceptionBitsULE ; fn from_unaligned (u : ExceptionBitsULE) -> Self { ExceptionBits :: from_integer (u . 0) } fn to_unaligned (self) -> ExceptionBitsULE { ExceptionBitsULE (self . to_integer ()) } }
};
}
