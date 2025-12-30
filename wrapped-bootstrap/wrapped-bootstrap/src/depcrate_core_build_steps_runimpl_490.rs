// Generated macro for impl_490 (impl)
macro_rules! Depcrate_core_build_steps_runimpl_490 {
() => {
// Module: crate::core::build_steps::run
// Provides: {"impl_490"}
// Dependencies: {}
impl Step for CollectLicenseMetadata { type Output = PathBuf ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/collect-license-metadata") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (CollectLicenseMetadata) ; } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let Some (reuse) = & builder . config . reuse else { panic ! ("REUSE is required to collect the license metadata") ; } ; let dest = builder . src . join ("license-metadata.json") ; let mut cmd = builder . tool_cmd (Tool :: CollectLicenseMetadata) ; cmd . env ("REUSE_EXE" , reuse) ; cmd . env ("DEST" , & dest) ; cmd . run (builder) ; dest } }
};
}
