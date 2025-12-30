// Generated macro for macro_51 (macro)
macro_rules! Depcrate_reactormacro_51 {
() => {
// Module: crate::reactor
// Provides: {"macro_51"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (windows)] { mod windows ; pub use windows :: Registration ; } else if # [cfg (any (target_vendor = "apple" , target_os = "freebsd" , target_os = "netbsd" , target_os = "openbsd" , target_os = "dragonfly" ,))] { mod kqueue ; pub use kqueue :: Registration ; } else if # [cfg (unix)] { mod unix ; pub use unix :: Registration ; } else { compile_error ! ("unsupported platform") ; } }
};
}
