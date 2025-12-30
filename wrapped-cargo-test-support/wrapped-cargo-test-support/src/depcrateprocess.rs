// Generated macro for process (function)
macro_rules! Depcrateprocess {
() => {
// Module: crate
// Provides: {"process"}
// Dependencies: {}
# [doc = " Run `$bin` in the test's environment, see [`ProcessBuilder`]"] # [doc = ""] # [doc = " For more on the test environment, see"] # [doc = " - [`paths::root`]"] # [doc = " - [`TestEnvCommandExt`]"] pub fn process < T : AsRef < OsStr > > (bin : T) -> ProcessBuilder { _process (bin . as_ref ()) }
};
}
