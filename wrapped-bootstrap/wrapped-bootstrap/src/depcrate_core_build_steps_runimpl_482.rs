// Generated macro for impl_482 (impl)
macro_rules! Depcrate_core_build_steps_runimpl_482 {
() => {
// Module: crate::core::build_steps::run
// Provides: {"impl_482"}
// Dependencies: {}
impl Step for BuildManifest { type Output = () ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/build-manifest") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (BuildManifest) ; } fn run (self , builder : & Builder < '_ >) { let mut cmd = builder . tool_cmd (Tool :: BuildManifest) ; let sign = builder . config . dist_sign_folder . as_ref () . unwrap_or_else (| | { panic ! ("\n\nfailed to specify `dist.sign-folder` in `bootstrap.toml`\n\n") }) ; let addr = builder . config . dist_upload_addr . as_ref () . unwrap_or_else (| | { panic ! ("\n\nfailed to specify `dist.upload-addr` in `bootstrap.toml`\n\n") }) ; let today = command ("date") . arg ("+%Y-%m-%d") . run_capture_stdout (builder) . stdout () ; cmd . arg (sign) ; cmd . arg (distdir (builder)) ; cmd . arg (today . trim ()) ; cmd . arg (addr) ; cmd . arg (& builder . config . channel) ; builder . create_dir (& distdir (builder)) ; cmd . run (builder) ; } }
};
}
