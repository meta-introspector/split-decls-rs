// Generated macro for impl_899 (impl)
macro_rules! Depcrate_core_builder_testsimpl_899 {
() => {
// Module: crate::core::builder::tests
// Provides: {"impl_899"}
// Dependencies: {}
impl < S : Step > From < S > for StepMetadata { fn from (step : S) -> Self { step . metadata () . expect ("step has no metadata") } }
};
}
