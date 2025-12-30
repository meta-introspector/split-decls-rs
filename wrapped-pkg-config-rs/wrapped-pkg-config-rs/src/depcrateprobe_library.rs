// Generated macro for probe_library (function)
macro_rules! Depcrateprobe_library {
() => {
// Module: crate
// Provides: {"probe_library"}
// Dependencies: {}
# [doc = " Simple shortcut for using all default options for finding a library."] pub fn probe_library (name : & str) -> Result < Library , Error > { Config :: new () . probe (name) }
};
}
