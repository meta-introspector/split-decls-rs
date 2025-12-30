// Generated macro for impl_555 (impl)
macro_rules! Depcrate_core_build_steps_synthetic_targetsimpl_555 {
() => {
// Module: crate::core::build_steps::synthetic_targets
// Provides: {"impl_555"}
// Dependencies: {}
impl Step for MirOptPanicAbortSyntheticTarget { type Output = TargetSelection ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . never () } fn run (self , builder : & Builder < '_ >) -> Self :: Output { create_synthetic_target (builder , self . compiler , "miropt-abort" , self . base , | spec | { spec . insert ("panic-strategy" . into () , "abort" . into ()) ; }) } }
};
}
