// Generated macro for drive (function)
macro_rules! Depcrate_driverdrive {
() => {
// Module: crate::driver
// Provides: {"drive"}
// Dependencies: {}
pub fn drive (addr : & str) { process :: exit (run_clippy (addr) . unwrap_or_else (| | { Command :: new ("rustc") . args (env :: args_os () . skip (2)) . status () . unwrap () . code () . unwrap () })) }
};
}
