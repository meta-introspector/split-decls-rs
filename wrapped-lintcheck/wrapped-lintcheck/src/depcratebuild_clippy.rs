// Generated macro for build_clippy (function)
macro_rules! Depcratebuild_clippy {
() => {
// Module: crate
// Provides: {"build_clippy"}
// Dependencies: {}
# [doc = " Builds clippy inside the repo to make sure we have a clippy executable we can use."] fn build_clippy (release_build : bool) -> String { let mut build_cmd = Command :: new ("cargo") ; build_cmd . args (["run" , "--bin=clippy-driver" , if release_build { "-r" } else { "" } , "--" , "--version" ,]) ; if release_build { build_cmd . env ("CARGO_PROFILE_RELEASE_DEBUG" , "true") ; } let output = build_cmd . stderr (Stdio :: inherit ()) . output () . unwrap () ; if ! output . status . success () { eprintln ! ("Error: Failed to compile Clippy!") ; std :: process :: exit (1) ; } String :: from_utf8_lossy (& output . stdout) . into_owned () }
};
}
