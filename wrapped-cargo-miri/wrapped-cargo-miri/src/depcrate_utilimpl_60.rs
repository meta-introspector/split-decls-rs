// Generated macro for impl_60 (impl)
macro_rules! Depcrate_utilimpl_60 {
() => {
// Module: crate::util
// Provides: {"impl_60"}
// Dependencies: {}
impl CrateRunEnv { # [doc = " Gather all the information we need."] pub fn collect (args : impl Iterator < Item = String > , capture_stdin : bool) -> Self { let args = args . collect () ; let env = env :: vars_os () . collect () ; let current_dir = env :: current_dir () . unwrap () . into_os_string () ; let mut stdin = Vec :: new () ; if capture_stdin { std :: io :: stdin () . lock () . read_to_end (& mut stdin) . expect ("cannot read stdin") ; } CrateRunEnv { args , env , current_dir , stdin } } }
};
}
