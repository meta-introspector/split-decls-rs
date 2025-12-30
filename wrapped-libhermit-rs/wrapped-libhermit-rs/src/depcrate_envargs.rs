// Generated macro for args (function)
macro_rules! Depcrate_envargs {
() => {
// Module: crate::env
// Provides: {"args"}
// Dependencies: {}
# [doc = " Returns the cmdline argument passed in after \"--\""] pub fn args () -> & 'static [String] { CLI . get () . unwrap () . args . as_slice () }
};
}
