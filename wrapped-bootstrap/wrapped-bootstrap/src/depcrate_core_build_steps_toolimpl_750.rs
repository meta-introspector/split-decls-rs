// Generated macro for impl_750 (impl)
macro_rules! Depcrate_core_build_steps_toolimpl_750 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"impl_750"}
// Dependencies: {}
impl Step for LibcxxVersionTool { type Output = LibcxxVersion ; const DEFAULT : bool = false ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . never () } fn run (self , builder : & Builder < '_ >) -> LibcxxVersion { let out_dir = builder . out . join (self . target . to_string ()) . join ("libcxx-version") ; let executable = out_dir . join (exe ("libcxx-version" , self . target)) ; if ! executable . exists () { if ! out_dir . exists () { t ! (fs :: create_dir_all (& out_dir)) ; } let compiler = builder . cxx (self . target) . unwrap () ; let mut cmd = command (compiler) ; cmd . arg ("-o") . arg (& executable) . arg (builder . src . join ("src/tools/libcxx-version/main.cpp")) ; cmd . run (builder) ; if ! executable . exists () { panic ! ("Something went wrong. {} is not present" , executable . display ()) ; } } let version_output = command (executable) . run_capture_stdout (builder) . stdout () ; let version_str = version_output . split_once ("version:") . unwrap () . 1 ; let version = version_str . trim () . parse :: < usize > () . unwrap () ; if version_output . starts_with ("libstdc++") { LibcxxVersion :: Gnu (version) } else if version_output . starts_with ("libc++") { LibcxxVersion :: Llvm (version) } else { panic ! ("Coudln't recognize the standard library version.") ; } } }
};
}
