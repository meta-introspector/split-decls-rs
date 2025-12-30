// Generated macro for impl_657 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_657 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_657"}
// Dependencies: {}
impl Step for CrateLibrustc { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . crate_or_deps ("rustc-main") . path ("compiler") } fn make_run (run : RunConfig < '_ >) { let builder = run . builder ; let host = run . build_triple () ; let build_compiler = builder . compiler (builder . top_stage - 1 , host) ; let crates = run . make_run_crates (Alias :: Compiler) ; builder . ensure (CrateLibrustc { build_compiler , target : run . target , crates }) ; } fn run (self , builder : & Builder < '_ >) { builder . std (self . build_compiler , self . target) ; builder . ensure (Crate { build_compiler : self . build_compiler , target : self . target , mode : Mode :: Rustc , crates : self . crates , }) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: test ("CrateLibrustc" , self . target) . built_by (self . build_compiler)) } }
};
}
