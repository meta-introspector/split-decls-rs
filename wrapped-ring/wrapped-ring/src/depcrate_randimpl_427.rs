// Generated macro for impl_427 (impl)
macro_rules! Depcrate_randimpl_427 {
() => {
// Module: crate::rand
// Provides: {"impl_427"}
// Dependencies: {}
# [cfg (any (all (feature = "less-safe-getrandom-custom-or-rdrand" , target_os = "none") , all (feature = "less-safe-getrandom-espidf" , target_os = "espidf") , target_os = "aix" , target_os = "android" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "fuchsia" , target_os = "haiku" , target_os = "hermit" , target_os = "hurd" , target_os = "horizon" , target_os = "illumos" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "redox" , target_os = "solaris" , target_os = "vita" , target_os = "windows" , target_os = "nto" , all (target_vendor = "apple" , any (target_os = "ios" , target_os = "macos" , target_os = "tvos" , target_os = "visionos" , target_os = "watchos" ,)) , all (target_arch = "wasm32" , any (target_os = "wasi" , all (target_os = "unknown" , feature = "wasm32_unknown_unknown_js"))) ,))] impl sealed :: SecureRandom for SystemRandom { # [inline (always)] fn fill_impl (& self , dest : & mut [u8] , _ : crate :: sealed :: Arg) -> Result < () , error :: Unspecified > { getrandom :: getrandom (dest) . map_err (| _ | error :: Unspecified) } }
};
}
