// Generated macro for obtain_spdx_document (function)
macro_rules! Depcrate_reuseobtain_spdx_document {
() => {
// Module: crate::reuse
// Provides: {"obtain_spdx_document"}
// Dependencies: {}
fn obtain_spdx_document (reuse_exe : & Path) -> Result < String , Error > { let output = Command :: new (reuse_exe) . args (& ["--include-submodules" , "spdx" , "--add-license-concluded" , "--creator-person=bors"]) . stdout (Stdio :: piped ()) . spawn () ? . wait_with_output () ? ; if ! output . status . success () { eprintln ! () ; eprintln ! ("Note that Rust requires some REUSE features that might not be present in the") ; eprintln ! ("release you're using. Make sure your REUSE release includes these PRs:") ; eprintln ! () ; eprintln ! (" - https://github.com/fsfe/reuse-tool/pull/623") ; eprintln ! () ; anyhow :: bail ! ("collecting licensing information with REUSE failed") ; } Ok (String :: from_utf8 (output . stdout) ?) }
};
}
