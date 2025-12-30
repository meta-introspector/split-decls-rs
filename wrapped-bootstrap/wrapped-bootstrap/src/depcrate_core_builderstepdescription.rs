// Generated macro for StepDescription (struct)
macro_rules! Depcrate_core_builderStepDescription {
() => {
// Module: crate::core::builder
// Provides: {"StepDescription"}
// Dependencies: {}
struct StepDescription { default : bool , is_host : bool , should_run : fn (ShouldRun < '_ >) -> ShouldRun < '_ > , make_run : fn (RunConfig < '_ >) , name : & 'static str , kind : Kind , }
};
}
