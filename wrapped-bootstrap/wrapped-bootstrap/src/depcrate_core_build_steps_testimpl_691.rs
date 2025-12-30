// Generated macro for impl_691 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_691 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_691"}
// Dependencies: {}
impl Step for CollectLicenseMetadata { type Output = PathBuf ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/collect-license-metadata") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (CollectLicenseMetadata) ; } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let Some (reuse) = & builder . config . reuse else { panic ! ("REUSE is required to collect the license metadata") ; } ; let dest = builder . src . join ("license-metadata.json") ; let mut cmd = builder . tool_cmd (Tool :: CollectLicenseMetadata) ; cmd . env ("REUSE_EXE" , reuse) ; cmd . env ("DEST" , & dest) ; cmd . env ("ONLY_CHECK" , "1") ; cmd . run (builder) ; dest } }
};
}
