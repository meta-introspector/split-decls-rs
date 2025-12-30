// Generated macro for freq (function)
macro_rules! Depcrate_envfreq {
() => {
// Module: crate::env
// Provides: {"freq"}
// Dependencies: {}
# [doc = " CPU Frequency in MHz if given through the -freq command-line parameter."] # [cfg (not (target_arch = "riscv64"))] pub fn freq () -> Option < u16 > { CLI . get () . unwrap () . freq }
};
}
